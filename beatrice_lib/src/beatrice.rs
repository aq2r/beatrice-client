use std::{
    ffi::CString,
    path::{Path, PathBuf},
    str::FromStr,
};

use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};

use crate::{BeatriceError, bindings::*};

struct BeatriceLibData {
    phone_extractor: *mut Beatrice20b1_PhoneExtractor,
    pitch_estimator: *mut Beatrice20b1_PitchEstimator,
    waveform_generator: *mut Beatrice20b1_WaveformGenerator,
    phone_context: *mut Beatrice20b1_PhoneContext1,
    pitch_context: *mut Beatrice20b1_PitchContext1,
    waveform_context: *mut Beatrice20b1_WaveformContext1,
}

struct BeatriceInfo {
    target_speaker: i32,
    formant_shift: f64,
    pitch_shift: f64,
    n_speakers: i32,
    average_source_pitch: f64,
    intonation_intensity: f64,
    pitch_correction: f64,
    pitch_correction_type: i32,

    input_sample_rate: f64,
    input_channel: u32,
    output_sample_rate: f64,
    output_channel: u32,
}

impl Default for BeatriceInfo {
    fn default() -> Self {
        Self {
            target_speaker: 0,
            formant_shift: 1.0,
            pitch_shift: 16.0,
            n_speakers: 0,
            average_source_pitch: 60.0,
            intonation_intensity: 1.0,
            pitch_correction: 0.0,
            pitch_correction_type: 0,

            input_sample_rate: 48000.0,
            output_sample_rate: 48000.0,
            input_channel: 2,
            output_channel: 2,
        }
    }
}

struct BeatriceModel {
    model_path: PathBuf,
    speaker_embeddings: Vec<f32>,
    formant_shift_embeddings: Vec<f32>,
}

pub struct Beatrice {
    model: Option<BeatriceModel>,
    info: BeatriceInfo,
    lib: BeatriceLibData,

    in_resampler: SincFixedIn<f32>,
    out_resampler: SincFixedIn<f32>,
}

impl Beatrice {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Beatrice {
        let lib = unsafe {
            BeatriceLibData {
                phone_extractor: Beatrice20b1_CreatePhoneExtractor(),
                pitch_estimator: Beatrice20b1_CreatePitchEstimator(),
                waveform_generator: Beatrice20b1_CreateWaveformGenerator(),
                phone_context: Beatrice20b1_CreatePhoneContext1(),
                pitch_context: Beatrice20b1_CreatePitchContext1(),
                waveform_context: Beatrice20b1_CreateWaveformContext1(),
            }
        };

        let info = BeatriceInfo::default();

        let in_resampler = SincFixedIn::<f32>::new(
            16000.0 / info.input_sample_rate,
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
        )
        .expect("Failed create resampler");

        let out_resampler = SincFixedIn::<f32>::new(
            info.output_sample_rate / 24000.0,
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
        )
        .expect("Failed create resampler");

        Beatrice {
            model: None,
            info,
            lib,

            in_resampler,
            out_resampler,
        }
    }

    // 現在20b1のみ対応
    pub fn load_model(&mut self, model_path: impl AsRef<Path>) -> Result<(), BeatriceError> {
        let model_path = model_path.as_ref();

        let create_cstring = |file_name: &str| {
            CString::from_str(
                model_path
                    .join(file_name)
                    .to_string_lossy()
                    .to_string()
                    .as_str(),
            )
        };

        // phone_extractor
        {
            let file_name = create_cstring("phone_extractor.bin")?;

            let result = unsafe {
                Beatrice20b1_ReadPhoneExtractorParameters(
                    self.lib.phone_extractor,
                    file_name.as_ptr(),
                )
            };

            if let Ok(err) = result.try_into() {
                return Err(err);
            }
        }

        // pitch_estimator
        {
            let file_name = create_cstring("pitch_estimator.bin")?;

            let result = unsafe {
                Beatrice20b1_ReadPitchEstimatorParameters(
                    self.lib.pitch_estimator,
                    file_name.as_ptr(),
                )
            };

            if let Ok(err) = result.try_into() {
                return Err(err);
            }
        }

        // waveform_generator
        {
            let file_name = create_cstring("waveform_generator.bin")?;

            let result = unsafe {
                Beatrice20b1_ReadWaveformGeneratorParameters(
                    self.lib.waveform_generator,
                    file_name.as_ptr(),
                )
            };

            if let Ok(err) = result.try_into() {
                return Err(err);
            }
        }

        // speaker_embeddings
        let mut speaker_embeddings = vec![];
        {
            let file_name = create_cstring("speaker_embeddings.bin")?;

            let result = unsafe {
                Beatrice20b1_ReadNSpeakers(file_name.as_ptr(), &mut self.info.n_speakers)
            };

            if let Ok(err) = result.try_into() {
                return Err(err);
            }

            let new_size =
                ((self.info.n_speakers + 1) as usize) * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS;
            speaker_embeddings.resize(new_size, 0.0_f32);

            let result = unsafe {
                Beatrice20b1_ReadSpeakerEmbeddings(
                    file_name.as_ptr(),
                    speaker_embeddings.as_mut_ptr(),
                )
            };

            if let Ok(err) = result.try_into() {
                return Err(err);
            }
        }

        // formant_shift_embeddings
        let mut formant_shift_embeddings = vec![];
        {
            let file_name = create_cstring("formant_shift_embeddings.bin")?;

            formant_shift_embeddings
                .resize(9 * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS, 0.0_f32);

            let result = unsafe {
                Beatrice20b1_ReadSpeakerEmbeddings(
                    file_name.as_ptr(),
                    formant_shift_embeddings.as_mut_ptr(),
                )
            };

            if let Ok(err) = result.try_into() {
                return Err(err);
            }
        }

        self.model = Some(BeatriceModel {
            model_path: model_path.to_path_buf(),
            speaker_embeddings,
            formant_shift_embeddings,
        });

        Ok(())
    }

    pub fn reset_context(&mut self) {
        unsafe {
            let phone_context = Beatrice20b1_CreatePhoneContext1();
            let pitch_context = Beatrice20b1_CreatePitchContext1();
            let waveform_context = Beatrice20b1_CreateWaveformContext1();

            let old_phone_ctx = self.lib.phone_context;
            let old_pitch_ctx = self.lib.pitch_context;
            let old_waveform_ctx = self.lib.waveform_context;

            self.lib.phone_context = phone_context;
            self.lib.pitch_context = pitch_context;
            self.lib.waveform_context = waveform_context;

            Beatrice20b1_DestroyPhoneContext1(old_phone_ctx);
            Beatrice20b1_DestroyPitchContext1(old_pitch_ctx);
            Beatrice20b1_DestroyWaveformContext1(old_waveform_ctx);
        }
    }

    pub fn infer(&mut self, input: &[f32]) -> Result<Vec<f32>, BeatriceError> {
        if self.model.is_none() {
            return Err(BeatriceError::ModelNotLoaded);
        }

        let deinterleaved = self.to_deinterleave(input);

        let mut result = vec![vec![]; 2];
        {
            let input_chunk = (self.info.input_sample_rate / 100.0) as usize;

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

        let mono = Self::stereo_to_mono_planer(result);

        let mut processed = vec![];
        for chunk in mono.chunks(BEATRICE_IN_HOP_LENGTH) {
            let mut buffer = [0.0; 160];

            buffer[..chunk.len()].copy_from_slice(chunk);
            processed.extend_from_slice(self.infer_slice(&buffer)?.as_ref());
        }

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

        let interleaved = self.to_interleave(result);
        Ok(interleaved)
    }

    pub fn get_model_path(&self) -> Option<&PathBuf> {
        match &self.model {
            Some(model) => Some(&model.model_path),
            None => None,
        }
    }

    /* Change Settings */
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

    pub fn set_speaker(&mut self, speaker: u32) -> Result<(), &str> {
        let speaker = speaker as i32;

        if (self.info.n_speakers - 1) < speaker {
            return Err("OutOfRange");
        }

        self.info.target_speaker = speaker;
        Ok(())
    }

    pub fn set_formant_shift(&mut self, formant_shift: f64) {
        self.info.formant_shift = formant_shift;
    }

    pub fn set_pitch_shift(&mut self, pitch_shift: f64) {
        self.info.pitch_shift = pitch_shift;
    }

    pub fn set_average_source_pitch(&mut self, average_source_pitch: f64) {
        self.info.average_source_pitch = average_source_pitch;
    }

    pub fn set_intonation_intensity(&mut self, intonation_intensity: f64) {
        self.info.intonation_intensity = intonation_intensity;
    }

    pub fn set_pitch_correction(&mut self, pitch_correction: f64) {
        self.info.pitch_correction = pitch_correction;
    }

    pub fn set_pitch_correction_type(&mut self, pitch_correction_type: i32) {
        self.info.pitch_correction_type = pitch_correction_type;
    }

    /* Private */
    fn infer_slice(
        &mut self,
        input: &[f32; BEATRICE_IN_HOP_LENGTH],
    ) -> Result<[f32; BEATRICE_OUT_HOP_LENGTH], BeatriceError> {
        let mut phone = [0.0_f32; BEATRICE_PHONE_CHANNELS];
        unsafe {
            Beatrice20b1_ExtractPhone1(
                self.lib.phone_extractor,
                input.as_ptr(),
                phone.as_mut_ptr(),
                self.lib.phone_context,
            )
        };

        let mut quantized_pitch = 0;
        let mut pitch_feature = [0.0_f32; 4];
        unsafe {
            Beatrice20b1_EstimatePitch1(
                self.lib.pitch_estimator,
                input.as_ptr(),
                &mut quantized_pitch,
                pitch_feature.as_mut_ptr(),
                self.lib.pitch_context,
            )
        };

        const KPITCH_BINS_PER_SEMITONE: f64 = BEATRICE_PITCH_BINS_PER_OCTAVE as f64 / 12.0;

        // PitchShift, IntonationIntensity
        let mut tmp_quantized_pitch = self.info.average_source_pitch
            + ((quantized_pitch as f64) - self.info.average_source_pitch)
                * self.info.intonation_intensity
            + KPITCH_BINS_PER_SEMITONE * self.info.pitch_shift;

        // PitchCorrection
        if self.info.pitch_correction != 0.0 {
            let before_pitch_correction = tmp_quantized_pitch;

            match self.info.pitch_correction_type {
                0 => {
                    let nearest_pitch = ((tmp_quantized_pitch / KPITCH_BINS_PER_SEMITONE).floor()
                        + 0.5)
                        * KPITCH_BINS_PER_SEMITONE;

                    let normalized_delta =
                        (tmp_quantized_pitch - nearest_pitch) * (2.0 / KPITCH_BINS_PER_SEMITONE);

                    if normalized_delta.abs() < 1e-4 {
                        tmp_quantized_pitch = nearest_pitch
                    } else {
                        tmp_quantized_pitch = nearest_pitch
                            + normalized_delta
                                * (normalized_delta.abs().powf(-self.info.pitch_correction))
                                * (KPITCH_BINS_PER_SEMITONE / 2.0);
                    }

                    debug_assert!(
                        (tmp_quantized_pitch
                            - (tmp_quantized_pitch / KPITCH_BINS_PER_SEMITONE).round()
                                * KPITCH_BINS_PER_SEMITONE)
                            .abs()
                            <= (before_pitch_correction
                                - (tmp_quantized_pitch / KPITCH_BINS_PER_SEMITONE).round()
                                    * KPITCH_BINS_PER_SEMITONE)
                                + 1e-4
                    )
                }

                1 => {
                    let nearest_pitch = (tmp_quantized_pitch / KPITCH_BINS_PER_SEMITONE).round()
                        * KPITCH_BINS_PER_SEMITONE;

                    let normalized_delta =
                        (tmp_quantized_pitch - nearest_pitch) * (2.0 / KPITCH_BINS_PER_SEMITONE);

                    if self.info.pitch_correction > (1.0 - 1e-4) {
                        tmp_quantized_pitch = nearest_pitch;
                    } else if normalized_delta >= 0.0 {
                        tmp_quantized_pitch = nearest_pitch
                            + normalized_delta.powf(1.0 / (1.0 - self.info.pitch_correction))
                                * (KPITCH_BINS_PER_SEMITONE / 2.0);
                    } else {
                        tmp_quantized_pitch = nearest_pitch
                            - (-normalized_delta).powf(1.0 / (1.0 - self.info.pitch_correction))
                                * (KPITCH_BINS_PER_SEMITONE / 2.0);
                    }
                    debug_assert!(
                        (tmp_quantized_pitch - nearest_pitch).abs()
                            <= (before_pitch_correction - nearest_pitch) + 1e-4
                    );
                }

                _ => {
                    debug_assert!(false);
                }
            }
        }

        quantized_pitch = {
            let rounded = tmp_quantized_pitch.round() as i32;
            rounded.clamp(1, BEATRICE_PITCH_BINS as i32 - 1)
        };

        // speaker
        let mut speaker = [0.0_f32; BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS];

        if let Some(self_model) = &mut self.model {
            unsafe {
                let src_start =
                    self.info.target_speaker as usize * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS;
                let src_slice = &self_model.speaker_embeddings
                    [src_start..src_start + BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS];

                std::ptr::copy_nonoverlapping(
                    src_slice.as_ptr(),
                    speaker.as_mut_ptr(),
                    BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS,
                );
            }

            let formant_shift_index = ((self.info.formant_shift * 2.0 + 4.0).round() as usize)
                * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS;

            for (i, take_speaker) in speaker
                .iter_mut()
                .enumerate()
                .take(BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS)
            {
                *take_speaker += self_model.formant_shift_embeddings[formant_shift_index + i];
            }
        } else {
            return Err(BeatriceError::ModelNotLoaded);
        }

        let mut output = [0.0; 240];

        unsafe {
            Beatrice20b1_GenerateWaveform1(
                self.lib.waveform_generator,
                phone.as_ptr(),
                &quantized_pitch,
                pitch_feature.as_ptr(),
                speaker.as_ptr(),
                output.as_mut_ptr(),
                self.lib.waveform_context,
            )
        };

        Ok(output)
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
        if self.info.output_channel == 2 {
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
        if self.info.input_channel == 2 {
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

unsafe impl Send for Beatrice {}

impl Drop for Beatrice {
    fn drop(&mut self) {
        unsafe {
            Beatrice20b1_DestroyPhoneExtractor(self.lib.phone_extractor);
            Beatrice20b1_DestroyPitchEstimator(self.lib.pitch_estimator);
            Beatrice20b1_DestroyWaveformGenerator(self.lib.waveform_generator);
            Beatrice20b1_DestroyPhoneContext1(self.lib.phone_context);
            Beatrice20b1_DestroyPitchContext1(self.lib.pitch_context);
            Beatrice20b1_DestroyWaveformContext1(self.lib.waveform_context)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_load_model() {
        let mut beatrice = Beatrice::new();
        beatrice
            .load_model("../beatrice_model/model_single")
            .expect("Failed load Model");
    }
}
