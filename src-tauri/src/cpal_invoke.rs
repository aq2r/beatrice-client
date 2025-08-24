use std::{
    sync::{mpsc, LazyLock, Mutex},
    thread,
};

use anyhow::Context;
use cpal::{
    traits::{DeviceTrait as _, HostTrait as _, StreamTrait as _},
    DeviceNameError, StreamConfig,
};
use ringbuf::{
    traits::{Consumer as _, Producer as _, Split as _},
    HeapRb,
};

use crate::beatrice_invoke::BEATRICE;

static INPUT_GAIN: Mutex<f32> = Mutex::new(1.0);
static OUTPUT_GAIN: Mutex<f32> = Mutex::new(1.0);

#[tauri::command]
pub async fn cpal_get_inputs() -> Result<Vec<String>, String> {
    let host = cpal::host_from_id(cpal::HostId::Wasapi).map_err(|err| err.to_string())?;

    let results: Result<Vec<String>, DeviceNameError> = host
        .input_devices()
        .map_err(|err| err.to_string())?
        .map(|i| i.name())
        .collect();

    results.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn cpal_get_outputs() -> Result<Vec<String>, String> {
    let host = cpal::host_from_id(cpal::HostId::Wasapi).map_err(|err| err.to_string())?;

    let results: Result<Vec<String>, DeviceNameError> = host
        .output_devices()
        .map_err(|err| err.to_string())?
        .map(|i| i.name())
        .collect();

    results.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn cpal_set_input_gain(gain: f32) {
    let mut input_gain = INPUT_GAIN.lock().unwrap();
    *input_gain = gain
}

#[tauri::command]
pub async fn cpal_set_output_gain(gain: f32) {
    let mut output_gain = OUTPUT_GAIN.lock().unwrap();
    *output_gain = gain
}

#[tauri::command]
pub async fn cpal_start_voice_changer(
    input_device_name: Option<String>,
    output_device_name: Option<String>,
    monitor_device_name: Option<String>,
) {
    static AUDIO_STOP_SENDER: LazyLock<Mutex<Option<mpsc::Sender<()>>>> =
        LazyLock::new(|| Mutex::new(None));

    {
        let mut audio_stop_sender = AUDIO_STOP_SENDER.lock().unwrap();
        if let Some(sender) = audio_stop_sender.as_ref() {
            let _ = sender.send(());
        }
        *audio_stop_sender = None;
    }

    let (sender, receiver) = mpsc::channel();
    {
        let mut audio_stop_sender = AUDIO_STOP_SENDER.lock().unwrap();
        *audio_stop_sender = Some(sender);
    }

    thread::spawn(move || {
        start_voice_changer(
            input_device_name,
            output_device_name,
            monitor_device_name,
            receiver,
        )
        .expect("msg");
    });
}

fn start_voice_changer(
    input_device_name: Option<String>,
    output_device_name: Option<String>,
    monitor_device_name: Option<String>,
    receiver: mpsc::Receiver<()>,
) -> anyhow::Result<()> {
    let Some(input_device_name) = input_device_name else {
        return Ok(());
    };

    let ring_size = 4096;

    let ring = HeapRb::new(ring_size);
    let (mut output_producer, mut output_consumer) = ring.split();
    let ring = HeapRb::new(ring_size);
    let (mut monitor_producer, mut monitor_consumer) = ring.split();

    let host = cpal::host_from_id(cpal::HostId::Wasapi)?;

    let input_stream = {
        let input_devices = host.input_devices()?.collect::<Vec<_>>();
        let Some(input_idx) = input_devices.iter().position(|device| {
            device.name().unwrap_or_else(|_| String::new()) == input_device_name
        }) else {
            return Ok(());
        };

        let input_device = host
            .input_devices()?
            .nth(input_idx)
            .context("input_device not found")?;
        let input_config = input_device.default_input_config()?;
        let input_stream_config = StreamConfig {
            channels: input_config.channels(),
            sample_rate: input_config.sample_rate(),
            buffer_size: cpal::BufferSize::Fixed(480),
        };

        {
            let mut beatrice = BEATRICE.lock().unwrap();

            if let Some(beatrice) = beatrice.as_mut() {
                beatrice.set_input_setting(
                    input_config.sample_rate().0.into(),
                    input_config.channels().into(),
                )?;
            };
        }

        input_device.build_input_stream(
            &input_stream_config,
            move |data: &[f32], _: &_| {
                let mut input_buffer = vec![0.0_f32; data.len()];
                input_buffer.copy_from_slice(data);

                let input_gain = { *INPUT_GAIN.lock().unwrap() };
                for i in input_buffer.iter_mut() {
                    *i *= input_gain;
                }

                let mut result = {
                    let mut beatrice = BEATRICE.lock().unwrap();

                    match beatrice.as_mut() {
                        Some(beatrice) => beatrice
                            .infer(&input_buffer)
                            .unwrap_or_else(|_| vec![0.0; data.len()]),

                        None => vec![0.0; data.len()],
                    }
                };

                let output_gain = { *OUTPUT_GAIN.lock().unwrap() };
                for i in result.iter_mut() {
                    *i *= output_gain;
                }

                let len = input_buffer.len().min(result.len());
                input_buffer[..len].copy_from_slice(&result[..len]);

                output_producer.push_slice(&input_buffer);
                monitor_producer.push_slice(&input_buffer);
            },
            |err| eprintln!("入力エラー: {err}"),
            None,
        )?
    };

    let output_stream = match output_device_name {
        Some(device_name) if device_name == "None" => None,
        None => None,

        Some(device_name) => {
            let output_devices = host.output_devices()?.collect::<Vec<_>>();
            let Some(output_idx) = output_devices
                .iter()
                .position(|device| device.name().unwrap_or_else(|_| String::new()) == device_name)
            else {
                return Ok(());
            };

            let output_device = host
                .output_devices()?
                .nth(output_idx)
                .context("output not found")?;
            let output_config = output_device.default_output_config()?;

            let output_stream_config = StreamConfig {
                channels: output_config.channels(),
                sample_rate: output_config.sample_rate(),
                buffer_size: cpal::BufferSize::Fixed(480),
            };

            {
                let mut beatrice = BEATRICE.lock().unwrap();

                if let Some(beatrice) = beatrice.as_mut() {
                    beatrice.set_output_setting(
                        output_config.sample_rate().0.into(),
                        output_config.channels().into(),
                    )?;
                };
            }

            Some(output_device.build_output_stream(
                &output_stream_config,
                move |data: &mut [f32], _: &_| {
                    output_consumer.pop_slice(data);
                },
                |err| eprintln!("出力エラー: {err}"),
                None,
            )?)
        }
    };

    let monitor_stream = match monitor_device_name {
        Some(device_name) if device_name == "None" => None,
        None => None,

        Some(device_name) => {
            let monitor_devices = host.output_devices()?.collect::<Vec<_>>();

            let Some(monitor_idx) = monitor_devices
                .iter()
                .position(|device| device.name().unwrap_or_else(|_| String::new()) == device_name)
            else {
                return Ok(());
            };

            let monitor_device = host
                .output_devices()?
                .nth(monitor_idx)
                .context("monitor_device not found")?;

            let monitor_config = monitor_device.default_output_config()?;

            let monitor_stream_config = StreamConfig {
                channels: monitor_config.channels(),
                sample_rate: monitor_config.sample_rate(),
                buffer_size: cpal::BufferSize::Fixed(480),
            };

            Some(monitor_device.build_output_stream(
                &monitor_stream_config,
                move |data: &mut [f32], _: &_| {
                    monitor_consumer.pop_slice(data);
                },
                |err| eprintln!("出力エラー: {err}"),
                None,
            )?)
        }
    };

    input_stream.play()?;

    if let Some(stream) = &output_stream {
        stream.play()?;
    }
    if let Some(stream) = &monitor_stream {
        stream.play()?;
    }

    while receiver.recv().is_ok() {
        input_stream.pause()?;

        if let Some(stream) = &output_stream {
            stream.pause()?;
        }
        if let Some(stream) = &monitor_stream {
            stream.pause()?;
        }
    }

    Ok(())
}
