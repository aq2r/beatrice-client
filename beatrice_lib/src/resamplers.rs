use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};

use crate::bindings::BEATRICE_OUT_HOP_LENGTH;

pub struct Resamplers {
    in_resampler: SincFixedIn<f32>,
    out_resampler: SincFixedIn<f32>,
    in_sample_rate: f64,
    _out_sample_rate: f64,
    input_channel: u32,
    output_channel: u32,
}

impl Resamplers {
    pub fn new(
        in_sample_rate: f64,
        out_sample_rate: f64,
        input_channel: u32,
        output_channel: u32,
    ) -> Result<Resamplers, rubato::ResamplerConstructionError> {
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
            480,
            2,
        )?;

        let out_resampler = SincFixedIn::<f32>::new(
            out_sample_rate / 24000.0,
            2.0,
            SincInterpolationParameters {
                sinc_len: 120,
                f_cutoff: 0.95,
                interpolation: SincInterpolationType::Linear,
                oversampling_factor: 256,
                window: WindowFunction::BlackmanHarris2,
            },
            240,
            2,
        )?;

        Ok(Resamplers {
            in_resampler,
            out_resampler,
            in_sample_rate,
            _out_sample_rate: out_sample_rate,
            input_channel,
            output_channel,
        })
    }

    pub fn convert_from_beatrice_input(&mut self, input: &[f32]) -> Vec<f32> {
        let deinterleaved = self.to_deinterleave(input);

        let mut result = vec![vec![]; 2];
        {
            let input_chunk = (self.in_sample_rate / 100.0) as usize;

            for (chunk_0, chunk_1) in deinterleaved[0]
                .chunks(input_chunk)
                .zip(deinterleaved[1].chunks(input_chunk))
            {
                if chunk_0.len() == input_chunk && chunk_1.len() == input_chunk {
                    let samples = [chunk_0, chunk_1];
                    let resampled = self.in_resampler.process(&samples, None).unwrap();

                    result[0].extend_from_slice(&resampled[0]);
                    result[1].extend_from_slice(&resampled[1]);
                }
            }
        }

        Self::stereo_to_mono_planer(result)
    }

    pub fn convert_from_beatrice_output(&mut self, processed: Vec<f32>) -> Vec<f32> {
        let stereo = Self::mono_to_stereo_planer(processed);

        let mut result = vec![vec![]; 2];
        for (chunk_0, chunk_1) in stereo[0]
            .chunks(BEATRICE_OUT_HOP_LENGTH)
            .zip(stereo[1].chunks(BEATRICE_OUT_HOP_LENGTH))
        {
            if chunk_0.len() == 240 && chunk_1.len() == 240 {
                let samples = [chunk_0, chunk_1];
                let resampled = self.out_resampler.process(&samples, None).unwrap();

                result[0].extend_from_slice(&resampled[0]);
                result[1].extend_from_slice(&resampled[1]);
            }
        }

        self.to_interleave(result)
    }

    pub fn set_input_setting(
        &mut self,
        sample_rate: f64,
        channel: usize,
    ) -> Result<(), rubato::ResamplerConstructionError> {
        let resampler = SincFixedIn::<f32>::new(
            16000.0 / sample_rate,
            2.0,
            SincInterpolationParameters {
                sinc_len: 256,
                f_cutoff: 0.95,
                interpolation: SincInterpolationType::Linear,
                oversampling_factor: 256,
                window: WindowFunction::BlackmanHarris2,
            },
            480,
            channel,
        )?;

        self.in_resampler = resampler;
        Ok(())
    }

    pub fn set_output_setting(
        &mut self,
        sample_rate: f64,
        channel: usize,
    ) -> Result<(), rubato::ResamplerConstructionError> {
        let resampler = SincFixedIn::<f32>::new(
            sample_rate / 24000.0,
            2.0,
            SincInterpolationParameters {
                sinc_len: 120,
                f_cutoff: 0.95,
                interpolation: SincInterpolationType::Linear,
                oversampling_factor: 256,
                window: WindowFunction::BlackmanHarris2,
            },
            240,
            channel,
        )?;

        self.out_resampler = resampler;
        Ok(())
    }

    fn stereo_to_mono_planer(mut input: Vec<Vec<f32>>) -> Vec<f32> {
        if input.len() == 2 {
            let left = input.remove(0);
            let right = input.remove(0);

            left.into_iter()
                .zip(right)
                .map(|(l, r)| (l + r) * 0.5)
                .collect()
        } else {
            input.remove(0)
        }
    }

    fn mono_to_stereo_planer(input: Vec<f32>) -> Vec<Vec<f32>> {
        vec![input.clone(), input]
    }

    fn to_interleave(&self, mut input: Vec<Vec<f32>>) -> Vec<f32> {
        if self.output_channel == 2 {
            let mut reinterleaved = Vec::with_capacity(input[0].len() * 2);

            for i in 0..input[0].len() {
                reinterleaved.push(input[0][i]); // L
                reinterleaved.push(input[1][i]); // R
            }

            reinterleaved
        } else {
            input.remove(0)
        }
    }

    fn to_deinterleave(&self, input: &[f32]) -> Vec<Vec<f32>> {
        if self.input_channel == 2 {
            let mut deinterleaved = vec![vec![]; 2];

            for (i, sample) in input.iter().enumerate() {
                deinterleaved[i % 2].push(*sample);
            }

            deinterleaved
        } else {
            vec![input.to_vec(), input.to_vec()]
        }
    }
}
