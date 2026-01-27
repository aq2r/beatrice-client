use std::{
    ffi::CString,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{beatrice::Beatrice, bindings::*, errors::BeatriceError, resampler::BeatriceResampler};

struct BeatriceLibData {
    phone_extractor: *mut Beatrice20b1_PhoneExtractor,
    pitch_estimator: *mut Beatrice20b1_PitchEstimator,
    waveform_generator: *mut Beatrice20b1_WaveformGenerator,
    phone_context: *mut Beatrice20b1_PhoneContext1,
    pitch_context: *mut Beatrice20b1_PitchContext1,
    waveform_context: *mut Beatrice20b1_WaveformContext1,
}

pub struct BeatriceInfo {
    pub target_speaker: i32,
    pub formant_shift: f64,
    pub pitch_shift: f64,
    pub n_speakers: i32,
    pub average_source_pitch: f64,
    pub intonation_intensity: f64,
    pub pitch_correction: f64,
    pub pitch_correction_type: i32,
}

impl Default for BeatriceInfo {
    fn default() -> Self {
        Self {
            target_speaker: 0,
            formant_shift: 0.0,
            pitch_shift: 0.0,
            n_speakers: 0,
            average_source_pitch: 60.0,
            intonation_intensity: 1.0,
            pitch_correction: 0.0,
            pitch_correction_type: 0,
        }
    }
}

struct BeatriceModel {
    model_path: PathBuf,
    speaker_embeddings: Vec<f32>,
    formant_shift_embeddings: Vec<f32>,
}

pub struct BeatriceBeta1 {
    model: Option<BeatriceModel>,
    pub info: BeatriceInfo,
    lib: BeatriceLibData,
    resampler: BeatriceResampler,
}

impl BeatriceBeta1 {
    pub fn new(
        in_sample_rate: f64,
        out_sample_rate: f64,
        in_channel: u32,
        out_channel: u32,
    ) -> Self {
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

        BeatriceBeta1 {
            model: None,
            info,
            lib,
            resampler: BeatriceResampler::new(
                in_sample_rate,
                out_sample_rate,
                in_channel,
                out_channel,
            ),
        }
    }

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

            let new_size = ((self.info.n_speakers + 1) as usize)
                * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize;
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

            formant_shift_embeddings.resize(
                (9 * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS) as usize,
                0.0_f32,
            );

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

    pub fn infer(&mut self, input: &[f32]) -> Result<Vec<f32>, BeatriceError> {
        if self.model.is_none() {
            return Err(BeatriceError::ModelNotLoaded);
        }

        let beatrice_input = self.resampler.convert_to_beatrice_input(input);

        let mut processed = vec![];
        for chunk in beatrice_input.chunks(BEATRICE_IN_HOP_LENGTH as usize) {
            let mut buffer = [0.0; 160];

            buffer[..chunk.len()].copy_from_slice(chunk);
            processed.extend_from_slice(self.infer_slice(&buffer)?.as_ref());
        }

        let output = self.resampler.convert_from_beatrice_output(&processed);
        Ok(output)
    }

    fn infer_slice(
        &mut self,
        input: &[f32; BEATRICE_IN_HOP_LENGTH as usize],
    ) -> Result<[f32; BEATRICE_OUT_HOP_LENGTH as usize], BeatriceError> {
        let mut phone = [0.0_f32; BEATRICE_20B1_PHONE_CHANNELS as usize];
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
            rounded.clamp(1, BEATRICE_20B1_PITCH_BINS as i32 - 1)
        };

        // speaker
        let mut speaker = [0.0_f32; BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize];

        if let Some(self_model) = &mut self.model {
            unsafe {
                let src_start = self.info.target_speaker as usize
                    * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize;
                let src_slice = &self_model.speaker_embeddings
                    [src_start..src_start + BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize];

                std::ptr::copy_nonoverlapping(
                    src_slice.as_ptr(),
                    speaker.as_mut_ptr(),
                    BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize,
                );
            }

            let formant_shift_index = ((self.info.formant_shift * 2.0 + 4.0).round() as usize)
                * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize;

            for (i, take_speaker) in speaker
                .iter_mut()
                .enumerate()
                .take(BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize)
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
}

unsafe impl Send for BeatriceBeta1 {}

impl Drop for BeatriceBeta1 {
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

impl Beatrice for BeatriceBeta1 {
    fn infer(&mut self, input: &[f32]) -> Result<Vec<f32>, BeatriceError> {
        self.infer(input)
    }

    fn get_model_path(&self) -> Option<&Path> {
        self.model.as_ref().map(|m| m.model_path.as_path())
    }

    fn get_n_speaker(&self) -> Option<i32> {
        self.model.as_ref().map(|_| self.info.n_speakers)
    }

    fn set_target_speaker(&mut self, speaker: u32) -> Result<(), BeatriceError> {
        let speaker = speaker as i32;

        if (self.info.n_speakers - 1) < speaker {
            return Err(BeatriceError::SpeakerOutOfRange);
        }

        self.info.target_speaker = speaker;
        Ok(())
    }

    fn set_formant_shift(&mut self, formant_shift: f64) {
        self.info.formant_shift = formant_shift;
    }

    fn set_pitch_shift(&mut self, pitch_shift: f64) {
        self.info.pitch_shift = pitch_shift;
    }

    fn set_average_source_pitch(&mut self, average_source_pitch: f64) {
        self.info.average_source_pitch = average_source_pitch;
    }

    fn set_intonation_intensity(&mut self, intonation_intensity: f64) {
        self.info.intonation_intensity = intonation_intensity;
    }

    fn set_pitch_correction(&mut self, pitch_correction: f64) {
        self.info.pitch_correction = pitch_correction;
    }

    fn set_pitch_correction_type(&mut self, pitch_correction_type: i32) {
        self.info.pitch_correction_type = pitch_correction_type;
    }

    fn set_min_source_pitch(&mut self, _min_source_pitch: f64) {}

    fn set_max_source_pitch(&mut self, _max_source_pitch: f64) {}

    fn set_vq_num_neighbors(&mut self, _vq_num_neighbors: i32) {}

    fn get_model_version(&self) -> &'static str {
        "2.0.0-beta.1"
    }
}
