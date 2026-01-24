use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};

pub struct BeatriceResampler {
    in_resampler: SincFixedIn<f32>,
    in_resampler_in_buff: Vec<Vec<f32>>,
    in_resampler_out_buff: Vec<Vec<f32>>,

    out_resampler: SincFixedIn<f32>,
    out_resampler_in_buff: Vec<Vec<f32>>,
    out_resampler_out_buff: Vec<Vec<f32>>,

    in_channel: u32,
    out_channel: u32,
}

impl BeatriceResampler {
    pub fn new(
        in_sample_rate: f64,
        out_sample_rate: f64,
        in_channel: u32,
        out_channel: u32,
    ) -> Self {
        let in_resampler = SincFixedIn::<f32>::new(
            16000.0 / in_sample_rate,
            2.0,
            SincInterpolationParameters {
                sinc_len: 256,
                f_cutoff: 0.95,
                interpolation: SincInterpolationType::Linear,
                oversampling_factor: 256,
                window: WindowFunction::BlackmanHarris2,
            },
            256,
            1,
        )
        .unwrap();
        let in_resampler_in_buff = in_resampler.input_buffer_allocate(false);
        let in_resampler_out_buff = in_resampler.output_buffer_allocate(false);

        let out_resampler = SincFixedIn::<f32>::new(
            out_sample_rate / 24000.0,
            2.0,
            SincInterpolationParameters {
                sinc_len: 256,
                f_cutoff: 0.95,
                interpolation: SincInterpolationType::Linear,
                oversampling_factor: 256,
                window: WindowFunction::BlackmanHarris2,
            },
            256,
            1,
        )
        .unwrap();
        let out_resampler_in_buff = out_resampler.input_buffer_allocate(false);
        let out_resampler_out_buff = out_resampler.output_buffer_allocate(false);

        Self {
            in_resampler,
            in_resampler_in_buff,
            in_resampler_out_buff,
            out_resampler,
            out_resampler_in_buff,
            out_resampler_out_buff,
            in_channel,
            out_channel,
        }
    }

    pub fn convert_to_beatrice_input(&mut self, input: &[f32]) -> Vec<f32> {
        self.in_resampler_in_buff[0].clear();
        self.in_resampler_out_buff[0].clear();

        match self.in_channel {
            1 => {
                self.in_resampler_in_buff[0].extend_from_slice(input);
            }
            2 => {
                for chunk in input.chunks_exact(2) {
                    self.in_resampler_in_buff[0].push((chunk[0] + chunk[1]) / 2.0);
                }
            }
            _ => panic!("in_channel must be 1 or 2"),
        };

        let (_input_used, output_produced) = self
            .in_resampler
            .process_into_buffer(
                &self.in_resampler_in_buff,
                &mut self.in_resampler_out_buff,
                None,
            )
            .unwrap();

        self.in_resampler_out_buff[0][..output_produced].to_vec()
    }

    pub fn convert_from_beatrice_output(&mut self, processed: &[f32]) -> Vec<f32> {
        self.out_resampler_in_buff[0].clear();
        self.out_resampler_out_buff[0].clear();

        self.out_resampler_in_buff[0].extend_from_slice(processed);

        let (_input_used, output_produced) = self
            .out_resampler
            .process_into_buffer(
                &self.out_resampler_in_buff,
                &mut self.out_resampler_out_buff,
                None,
            )
            .unwrap();

        match self.out_channel {
            1 => self.out_resampler_out_buff[0][..output_produced].to_vec(),
            2 => {
                let mut out = Vec::with_capacity(output_produced * 2);
                for s in &self.out_resampler_out_buff[0][..output_produced] {
                    out.push(*s);
                    out.push(*s);
                }
                out
            }
            _ => panic!("in_channel must be 1 or 2"),
        }
    }
}
