use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};

pub struct BeatriceResampler {
    in_resampler: SincFixedIn<f32>,
    out_resampler: SincFixedIn<f32>,

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
                sinc_len: 128,
                f_cutoff: 0.9,
                interpolation: SincInterpolationType::Linear,
                oversampling_factor: 64,
                window: WindowFunction::BlackmanHarris2,
            },
            (in_sample_rate / 100.0).round() as usize,
            1,
        )
        .unwrap();

        let out_resampler = SincFixedIn::<f32>::new(
            out_sample_rate / 24000.0,
            2.0,
            SincInterpolationParameters {
                sinc_len: 128,
                f_cutoff: 0.9,
                interpolation: SincInterpolationType::Linear,
                oversampling_factor: 64,
                window: WindowFunction::BlackmanHarris2,
            },
            240,
            1,
        )
        .unwrap();

        Self {
            in_resampler,
            out_resampler,
            in_channel,
            out_channel,
        }
    }

    pub fn convert_to_beatrice_input(&mut self, input: &[f32]) -> Vec<f32> {
        let mut mono = vec![];
        match self.in_channel {
            1 => mono.extend_from_slice(input),
            2 => {
                for chunk in input.chunks_exact(2) {
                    mono.push((chunk[0] + chunk[1]) / 2.0);
                }
            }
            _ => panic!("in_channel must be 1 or 2"),
        };

        self.in_resampler.process(&[mono], None).unwrap().remove(0)
    }

    pub fn convert_from_beatrice_output(&mut self, processed: &[f32]) -> Vec<f32> {
        let out = match self.out_resampler.process(&[processed], None) {
            Ok(v) => v,
            Err(e) => {
                dbg!(e);
                return Vec::new();
            }
        };

        let mono = &out[0];

        match self.out_channel {
            1 => mono.clone(),
            2 => {
                let mut stereo = Vec::with_capacity(mono.len() * 2);
                for &s in mono {
                    stereo.push(s);
                    stereo.push(s);
                }
                stereo
            }
            _ => panic!("out_channel must be 1 or 2"),
        }
    }
}
