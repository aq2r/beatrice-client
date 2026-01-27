use anyhow::Context;
use beatrice_lib::Beatrice;
use cpal::{
    StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait as _},
};
use ringbuf::{
    HeapRb,
    traits::{Consumer as _, Producer as _, Split as _},
};
use std::{
    sync::{LazyLock, Mutex, mpsc},
    thread,
    time::Duration,
};
use tauri::Emitter as _;

pub static BEATRICE: LazyLock<Mutex<Option<Box<dyn Beatrice>>>> =
    LazyLock::new(|| Mutex::new(None));

#[tauri::command]
pub async fn cpal_get_inputs() -> Result<Vec<String>, String> {
    let host = cpal::host_from_id(cpal::HostId::Wasapi).map_err(|err| err.to_string())?;

    let inputs = host.input_devices().map_err(|e| e.to_string())?;

    let mut names = Vec::new();
    for device in inputs {
        let desc = device.name().map_err(|e| e.to_string())?;
        names.push(desc);
    }

    Ok(names)
}

#[tauri::command]
pub async fn cpal_get_outputs() -> Result<Vec<String>, String> {
    let host = cpal::default_host();

    let outputs = host.output_devices().map_err(|e| e.to_string())?;

    let mut names = Vec::new();
    for device in outputs {
        let desc = device.name().map_err(|e| e.to_string())?;
        names.push(desc);
    }

    Ok(names)
}

static INPUT_GAIN: Mutex<f32> = Mutex::new(1.0);
#[tauri::command]
pub async fn cpal_set_input_gain(gain: f32) {
    let mut lock = INPUT_GAIN.lock().unwrap();
    *lock = gain
}

static OUTPUT_GAIN: Mutex<f32> = Mutex::new(1.0);
#[tauri::command]
pub async fn cpal_set_output_gain(gain: f32) {
    let mut lock = OUTPUT_GAIN.lock().unwrap();
    *lock = gain
}

static MONITOR_GAIN: Mutex<f32> = Mutex::new(1.0);
#[tauri::command]
pub async fn cpal_set_monitor_gain(gain: f32) {
    let mut lock = MONITOR_GAIN.lock().unwrap();
    *lock = gain
}

static INPUT_THRESHOLD: Mutex<f32> = Mutex::new(1.0);
static MIC_LEVEL: Mutex<f32> = Mutex::new(1.0);
#[tauri::command]
pub async fn cpal_set_input_threshold(threshold: f32) {
    let mut lock = INPUT_THRESHOLD.lock().unwrap();
    *lock = threshold
}

#[tauri::command]
pub async fn cpal_start_voice_changer(
    app_handle: tauri::AppHandle,
    model_path: String,
    input_device_name: Option<String>,
    output_device_name: Option<String>,
    monitor_device_name: Option<String>,
) {
    thread::spawn(move || {
        static IS_EXEC: Mutex<bool> = Mutex::new(false);
        let is_exec = {
            let mut lock = IS_EXEC.lock().unwrap();
            let value = *lock;
            *lock = true;

            value
        };

        if is_exec {
            return;
        }

        loop {
            let mic_level = {
                let lock = MIC_LEVEL.lock().unwrap();
                *lock
            };

            let _ = app_handle.emit("mic-level", mic_level);
            thread::sleep(Duration::from_millis(50));
        }
    });

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

    thread::spawn(move || -> anyhow::Result<()> {
        let (Some(input_device_name), Some(output_device_name)) =
            (input_device_name, output_device_name)
        else {
            return Ok(());
        };

        let ring_size = 4096;

        let (mut output_producer, mut output_consumer) = HeapRb::new(ring_size).split();
        let (mut monitor_producer, mut monitor_consumer) = HeapRb::new(ring_size).split();

        let host = cpal::host_from_id(cpal::HostId::Wasapi)?;

        // input
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

        // output
        let output_devices = host.output_devices()?.collect::<Vec<_>>();
        let Some(output_idx) = output_devices.iter().position(|device| {
            device.name().unwrap_or_else(|_| String::new()) == output_device_name
        }) else {
            return Ok(());
        };

        let output_device = host
            .output_devices()?
            .nth(output_idx)
            .context("output not found")?;
        let output_config = output_device.default_output_config()?;

        // monitor
        let monitor_devices = host.output_devices()?.collect::<Vec<_>>();
        let monitor_device = match monitor_device_name {
            Some(device_name) if device_name == "None" => None,
            Some(device_name) => {
                let Some(monitor_idx) = monitor_devices.iter().position(|device| {
                    device.name().unwrap_or_else(|_| String::new()) == device_name
                }) else {
                    return Ok(());
                };

                let monitor_device = host
                    .output_devices()?
                    .nth(monitor_idx)
                    .context("output not found")?;

                Some(monitor_device)
            }
            None => None,
        };

        let beatrice = beatrice_lib::new(
            model_path,
            input_config.sample_rate().0.into(),
            output_config.sample_rate().0.into(),
            input_config.channels().into(),
            output_config.channels().into(),
        )?;

        {
            let mut lock = BEATRICE.lock().unwrap();
            *lock = Some(beatrice)
        }

        let input_stream = {
            let input_stream_config = StreamConfig {
                channels: input_config.channels(),
                sample_rate: input_config.sample_rate(),
                buffer_size: cpal::BufferSize::Fixed(480),
            };

            input_device.build_input_stream(
                &input_stream_config,
                move |data: &[f32], _: &_| {
                    let mut input_buffer = vec![0.0_f32; data.len()];
                    input_buffer.copy_from_slice(data);

                    let input_gain = { *INPUT_GAIN.lock().unwrap() };
                    for i in input_buffer.iter_mut() {
                        *i *= input_gain;
                    }

                    let sum_squares: f32 = input_buffer.iter().map(|v| v * v).sum();
                    let rms = (sum_squares / input_buffer.len() as f32).sqrt();
                    {
                        let mut lock = MIC_LEVEL.lock().unwrap();
                        *lock = rms.powf(0.3);
                    }

                    let result = {
                        let input_threshold = {
                            let lock = INPUT_THRESHOLD.lock().unwrap();
                            *lock
                        };
                        let mic_level = {
                            let lock = MIC_LEVEL.lock().unwrap();
                            *lock
                        };

                        match input_threshold < mic_level {
                            true => {
                                let mut beatrice = BEATRICE.lock().unwrap();

                                match beatrice.as_mut() {
                                    Some(beatrice) => beatrice
                                        .infer(&input_buffer)
                                        .unwrap_or_else(|_| vec![0.0; data.len()]),

                                    None => vec![0.0; data.len()],
                                }
                            }
                            false => vec![0.0; data.len()],
                        }
                    };

                    output_producer.push_slice(&result);
                    monitor_producer.push_slice(&result);
                },
                |err| eprintln!("入力エラー: {err}"),
                None,
            )?
        };

        let output_stream = {
            let output_stream_config = StreamConfig {
                channels: output_config.channels(),
                sample_rate: output_config.sample_rate(),
                buffer_size: cpal::BufferSize::Fixed(480),
            };

            output_device.build_output_stream(
                &output_stream_config,
                move |data: &mut [f32], _: &_| {
                    let mut output_buffer = vec![0.0_f32; data.len()];
                    output_consumer.pop_slice(&mut output_buffer);

                    let output_gain = { *OUTPUT_GAIN.lock().unwrap() };
                    for i in output_buffer.iter_mut() {
                        *i *= output_gain;
                    }

                    data.copy_from_slice(&output_buffer);
                },
                |err| eprintln!("出力エラー: {err}"),
                None,
            )?
        };

        let monitor_stream = match monitor_device {
            Some(device) => {
                let monitor_stream_config = StreamConfig {
                    channels: output_config.channels(),
                    sample_rate: output_config.sample_rate(),
                    buffer_size: cpal::BufferSize::Fixed(480),
                };

                Some(device.build_output_stream(
                    &monitor_stream_config,
                    move |data: &mut [f32], _: &_| {
                        let mut output_buffer = vec![0.0_f32; data.len()];
                        monitor_consumer.pop_slice(&mut output_buffer);

                        let output_gain = { *MONITOR_GAIN.lock().unwrap() };
                        for i in output_buffer.iter_mut() {
                            *i *= output_gain;
                        }

                        data.copy_from_slice(&output_buffer);
                    },
                    |err| eprintln!("出力エラー: {err}"),
                    None,
                )?)
            }
            None => None,
        };

        input_stream.play()?;
        output_stream.play()?;
        if let Some(stream) = &monitor_stream {
            stream.play()?;
        }

        while receiver.recv().is_ok() {
            input_stream.pause()?;
            output_stream.pause()?;

            if let Some(stream) = &monitor_stream {
                stream.pause()?;
            }
        }

        Ok(())
    });
}
