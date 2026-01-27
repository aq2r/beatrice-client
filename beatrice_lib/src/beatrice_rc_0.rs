use std::{
    ffi::CString,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{beatrice::Beatrice, bindings::*, errors::BeatriceError, resampler::BeatriceResampler};

struct BeatriceLibData {
    phone_extractor: *mut Beatrice20rc0_PhoneExtractor,
    pitch_estimator: *mut Beatrice20rc0_PitchEstimator,
    waveform_generator: *mut Beatrice20rc0_WaveformGenerator,
    embedding_setter: *mut Beatrice20rc0_EmbeddingSetter,

    codebooks: Vec<f32>,
    additive_speaker_embeddings: Vec<f32>,
    formant_shift_embeddings: Vec<f32>,
    key_value_speaker_embeddings: Vec<f32>,

    phone_context: *mut Beatrice20rc0_PhoneContext1,
    pitch_context: *mut Beatrice20rc0_PitchContext1,
    waveform_context: *mut Beatrice20rc0_WaveformContext1,
    embedding_context: *mut Beatrice20rc0_EmbeddingContext,

    key_value_speaker_embedding_set_count: i32,
    is_ready_to_set_speaker: bool,
}

#[derive(Debug)]
pub struct BeatriceInfo {
    pub target_speaker: i32,
    pub formant_shift: f64,
    pub pitch_shift: f64,
    pub n_speakers: i32,
    pub average_source_pitch: f64,
    pub intonation_intensity: f64,
    pub pitch_correction: f64,
    pub pitch_correction_type: i32,
    pub min_source_pitch: f64,
    pub max_source_pitch: f64,
    pub vq_num_neighbors: i32,
}

impl Default for BeatriceInfo {
    fn default() -> Self {
        Self {
            target_speaker: 0,
            formant_shift: 0.0,
            pitch_shift: 0.0,
            n_speakers: 0,
            average_source_pitch: 52.0,
            intonation_intensity: 1.0,
            pitch_correction: 0.0,
            pitch_correction_type: 0,
            min_source_pitch: 33.125,
            max_source_pitch: 80.875,
            vq_num_neighbors: 0,
        }
    }
}

#[derive(Debug)]
struct BeatriceModel {
    model_path: PathBuf,
}

pub struct BeatriceRC0 {
    model: Option<BeatriceModel>,
    pub info: BeatriceInfo,
    lib: BeatriceLibData,
    resamplers: BeatriceResampler,
}

impl BeatriceRC0 {
    #[allow(clippy::new_without_default)]
    pub fn new(
        in_sample_rate: f64,
        out_sample_rate: f64,
        in_channel: u32,
        out_channel: u32,
    ) -> BeatriceRC0 {
        let lib = unsafe {
            BeatriceLibData {
                phone_extractor: Beatrice20rc0_CreatePhoneExtractor(),
                pitch_estimator: Beatrice20rc0_CreatePitchEstimator(),
                waveform_generator: Beatrice20rc0_CreateWaveformGenerator(),
                embedding_setter: Beatrice20rc0_CreateEmbeddingSetter(),

                codebooks: vec![],
                additive_speaker_embeddings: vec![],
                formant_shift_embeddings: vec![],
                key_value_speaker_embeddings: vec![],

                phone_context: Beatrice20rc0_CreatePhoneContext1(),
                pitch_context: Beatrice20rc0_CreatePitchContext1(),
                waveform_context: Beatrice20rc0_CreateWaveformContext1(),
                embedding_context: Beatrice20rc0_CreateEmbeddingContext(),

                key_value_speaker_embedding_set_count: 0,
                is_ready_to_set_speaker: false,
            }
        };

        let info = BeatriceInfo::default();

        BeatriceRC0 {
            model: None,
            info,
            lib,
            resamplers: BeatriceResampler::new(
                in_sample_rate,
                out_sample_rate,
                in_channel,
                out_channel,
            ),
        }
    }

    pub fn infer(&mut self, input: &[f32]) -> Result<Vec<f32>, BeatriceError> {
        if self.model.is_none() {
            return Err(BeatriceError::ModelNotLoaded);
        }

        let beatrice_input = self.resamplers.convert_to_beatrice_input(input);

        let mut processed = vec![];
        for chunk in beatrice_input.chunks(BEATRICE_IN_HOP_LENGTH as usize) {
            let mut buffer = [0.0; 160];

            buffer[..chunk.len()].copy_from_slice(chunk);
            processed.extend_from_slice(self.infer_slice(&buffer)?.as_ref());
        }

        let output = self.resamplers.convert_from_beatrice_output(&processed);
        Ok(output)
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
                Beatrice20rc0_ReadPhoneExtractorParameters(
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
                Beatrice20rc0_ReadPitchEstimatorParameters(
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
                Beatrice20rc0_ReadWaveformGeneratorParameters(
                    self.lib.waveform_generator,
                    file_name.as_ptr(),
                )
            };

            if let Ok(err) = result.try_into() {
                return Err(err);
            }
        }

        // embedding_setter
        {
            let file_name = create_cstring("embedding_setter.bin")?;

            let result = unsafe {
                Beatrice20rc0_ReadEmbeddingSetterParameters(
                    self.lib.embedding_setter,
                    file_name.as_ptr(),
                )
            };

            if let Ok(err) = result.try_into() {
                return Err(err);
            }
        }

        // speaker_embeddings
        {
            let file_name = create_cstring("speaker_embeddings.bin")?;

            let result = unsafe {
                Beatrice20rc0_ReadNSpeakers(file_name.as_ptr(), &mut self.info.n_speakers)
            };

            if let Ok(err) = result.try_into() {
                return Err(err);
            }

            let n_speakers_plus_1 = (self.info.n_speakers + 1) as usize;

            self.lib.codebooks.resize(
                n_speakers_plus_1
                    * (BEATRICE_20RC0_CODEBOOK_SIZE * BEATRICE_20RC0_PHONE_CHANNELS) as usize,
                0.0,
            );

            self.lib.additive_speaker_embeddings.resize(
                n_speakers_plus_1 * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize,
                0.0,
            );

            self.lib.formant_shift_embeddings.resize(
                9 * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize,
                0.0,
            );

            self.lib.key_value_speaker_embeddings.resize(
                n_speakers_plus_1
                    * (BEATRICE_20RC0_KV_LENGTH * BEATRICE_20RC0_KV_SPEAKER_EMBEDDING_CHANNELS)
                        as usize,
                0.0,
            );

            let result = unsafe {
                Beatrice20rc0_ReadSpeakerEmbeddings(
                    file_name.as_ptr(),
                    self.lib.codebooks.as_mut_ptr(),
                    self.lib.additive_speaker_embeddings.as_mut_ptr(),
                    self.lib.formant_shift_embeddings.as_mut_ptr(),
                    self.lib.key_value_speaker_embeddings.as_mut_ptr(),
                )
            };

            if let Ok(err) = result.try_into() {
                return Err(err);
            }
        }

        self.lib.is_ready_to_set_speaker = true;
        self.set_target_speaker(0)?;
        while self.set_key_value_speaker_embedding() {}

        self.model = Some(BeatriceModel {
            model_path: model_path.to_path_buf(),
        });

        Ok(())
    }

    pub fn set_min_source_pitch(&mut self, min_source_pitch: f64) {
        self.info.min_source_pitch = min_source_pitch.clamp(0.0, 128.0);

        unsafe {
            Beatrice20rc0_SetMinQuantizedPitch(
                self.lib.pitch_context,
                (((self.info.min_source_pitch - 33.0)
                    * (BEATRICE_PITCH_BINS_PER_OCTAVE as f64 / 12.0)) as i32)
                    .clamp(1, BEATRICE_20RC0_PITCH_BINS as i32 - 1),
            )
        };
    }

    pub fn set_max_source_pitch(&mut self, max_source_pitch: f64) {
        self.info.max_source_pitch = max_source_pitch.clamp(0.0, 128.0);

        unsafe {
            Beatrice20rc0_SetMaxQuantizedPitch(
                self.lib.pitch_context,
                (((self.info.max_source_pitch - 33.0)
                    * (BEATRICE_PITCH_BINS_PER_OCTAVE as f64 / 12.0)) as i32)
                    .clamp(1, BEATRICE_20RC0_PITCH_BINS as i32 - 1),
            )
        };
    }

    pub fn set_target_speaker(&mut self, speaker: u32) -> Result<(), BeatriceError> {
        let new_target_speaker_id = speaker as i32;

        if (self.info.n_speakers + 1) <= new_target_speaker_id {
            return Err(BeatriceError::SpeakerOutOfRange);
        }

        // assert
        {
            let n_speakers_plus1 = (self.info.n_speakers + 1) as usize;

            debug_assert!(
                self.lib.codebooks.len()
                    == n_speakers_plus1
                        * (BEATRICE_20RC0_CODEBOOK_SIZE * BEATRICE_20RC0_PHONE_CHANNELS) as usize
            );

            debug_assert!(
                self.lib.additive_speaker_embeddings.len()
                    == n_speakers_plus1 * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize
            );

            debug_assert!(
                self.lib.key_value_speaker_embeddings.len()
                    == n_speakers_plus1
                        * (BEATRICE_20RC0_KV_LENGTH * BEATRICE_20RC0_KV_SPEAKER_EMBEDDING_CHANNELS)
                            as usize
            );
        }

        unsafe {
            {
                let offset = new_target_speaker_id as usize
                    * (BEATRICE_20RC0_CODEBOOK_SIZE * BEATRICE_20RC0_PHONE_CHANNELS) as usize;

                Beatrice20rc0_SetCodebook(
                    self.lib.phone_context,
                    self.lib.codebooks.as_ptr().add(offset),
                )
            }

            {
                let offset = new_target_speaker_id as usize
                    * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize;

                Beatrice20rc0_SetAdditiveSpeakerEmbedding(
                    self.lib.embedding_setter,
                    self.lib.additive_speaker_embeddings.as_ptr().add(offset),
                    self.lib.embedding_context,
                    self.lib.waveform_context,
                );
            }

            {
                let offset = new_target_speaker_id as usize
                    * (BEATRICE_20RC0_KV_LENGTH * BEATRICE_20RC0_KV_SPEAKER_EMBEDDING_CHANNELS)
                        as usize;

                Beatrice20rc0_RegisterKeyValueSpeakerEmbedding(
                    self.lib.embedding_setter,
                    self.lib.key_value_speaker_embeddings.as_ptr().add(offset),
                    self.lib.embedding_context,
                );
            }
        };

        self.info.target_speaker = new_target_speaker_id;
        self.lib.key_value_speaker_embedding_set_count = 0;

        Ok(())
    }

    pub fn set_formant_shift(&mut self, formant_shift: f64) {
        self.info.formant_shift = formant_shift;

        let index = (self.info.formant_shift * 2.0 + 4.0).round() as isize;

        debug_assert!((0..9).contains(&index));
        debug_assert!(
            self.lib.formant_shift_embeddings.len()
                == 9 * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize
        );

        unsafe {
            Beatrice20rc0_SetFormantShiftEmbedding(
                self.lib.embedding_setter,
                self.lib
                    .formant_shift_embeddings
                    .as_ptr()
                    .offset(index * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as isize),
                self.lib.embedding_context,
                self.lib.waveform_context,
            );
        }
    }

    fn set_key_value_speaker_embedding(&mut self) -> bool {
        if self.lib.key_value_speaker_embedding_set_count < BEATRICE_20RC0_N_BLOCKS as i32 {
            let count = self.lib.key_value_speaker_embedding_set_count;
            self.lib.key_value_speaker_embedding_set_count += 1;

            unsafe {
                Beatrice20rc0_SetKeyValueSpeakerEmbedding(
                    self.lib.embedding_setter,
                    count,
                    self.lib.embedding_context,
                    self.lib.waveform_context,
                );
            }

            return true;
        }

        false
    }

    fn infer_slice(
        &mut self,
        input: &[f32; BEATRICE_IN_HOP_LENGTH as usize],
    ) -> Result<[f32; BEATRICE_OUT_HOP_LENGTH as usize], BeatriceError> {
        self.set_key_value_speaker_embedding();

        // ExtractPhone
        let mut phone = [0.0_f32; BEATRICE_20RC0_PHONE_CHANNELS as usize];
        unsafe {
            Beatrice20rc0_ExtractPhone1(
                self.lib.phone_extractor,
                input.as_ptr(),
                phone.as_mut_ptr(),
                self.lib.phone_context,
            )
        };

        let mut quantized_pitch = 0;
        let mut pitch_feature = [0.0_f32; 4];
        unsafe {
            Beatrice20rc0_EstimatePitch1(
                self.lib.pitch_estimator,
                input.as_ptr(),
                &mut quantized_pitch,
                pitch_feature.as_mut_ptr(),
                self.lib.pitch_context,
            );
        }

        const KPITCH_BINS_PER_SEMITONE: f64 = BEATRICE_PITCH_BINS_PER_OCTAVE as f64 / 12.0;

        // PitchShift, IntonationIntensity
        let mut tmp_quantized_pitch = self.info.average_source_pitch
            + (quantized_pitch as f64 - self.info.average_source_pitch)
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
                                .abs()
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
                            <= (before_pitch_correction - nearest_pitch).abs() + 1e-4
                    );
                }

                _ => {
                    debug_assert!(false);
                }
            }
        }

        quantized_pitch = {
            let rounded = tmp_quantized_pitch.round() as i32;
            rounded.clamp(1, BEATRICE_20RC0_PITCH_BINS as i32 - 1)
        };

        let mut output = [0.0; 240];
        unsafe {
            Beatrice20rc0_GenerateWaveform1(
                self.lib.waveform_generator,
                phone.as_ptr(),
                &quantized_pitch,
                pitch_feature.as_ptr(),
                output.as_mut_ptr(),
                self.lib.waveform_context,
            );
        }

        Ok(output)
    }
}

unsafe impl Send for BeatriceRC0 {}

impl Drop for BeatriceRC0 {
    fn drop(&mut self) {
        unsafe {
            Beatrice20rc0_DestroyPhoneExtractor(self.lib.phone_extractor);
            Beatrice20rc0_DestroyPitchEstimator(self.lib.pitch_estimator);
            Beatrice20rc0_DestroyWaveformGenerator(self.lib.waveform_generator);
            Beatrice20rc0_DestroyEmbeddingSetter(self.lib.embedding_setter);

            Beatrice20rc0_DestroyPhoneContext1(self.lib.phone_context);
            Beatrice20rc0_DestroyPitchContext1(self.lib.pitch_context);
            Beatrice20rc0_DestroyWaveformContext1(self.lib.waveform_context);
            Beatrice20rc0_DestroyEmbeddingContext(self.lib.embedding_context);
        }
    }
}

impl Beatrice for BeatriceRC0 {
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
        let new_target_speaker_id = speaker as i32;

        if (self.info.n_speakers + 1) <= new_target_speaker_id {
            return Err(BeatriceError::SpeakerOutOfRange);
        }

        // assert
        {
            let n_speakers_plus1 = (self.info.n_speakers + 1) as usize;

            debug_assert!(
                self.lib.codebooks.len()
                    == n_speakers_plus1
                        * (BEATRICE_20RC0_CODEBOOK_SIZE * BEATRICE_20RC0_PHONE_CHANNELS) as usize
            );

            debug_assert!(
                self.lib.additive_speaker_embeddings.len()
                    == n_speakers_plus1 * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize
            );

            debug_assert!(
                self.lib.key_value_speaker_embeddings.len()
                    == n_speakers_plus1
                        * (BEATRICE_20RC0_KV_LENGTH * BEATRICE_20RC0_KV_SPEAKER_EMBEDDING_CHANNELS)
                            as usize
            );
        }

        unsafe {
            {
                let offset = new_target_speaker_id as usize
                    * (BEATRICE_20RC0_CODEBOOK_SIZE * BEATRICE_20RC0_PHONE_CHANNELS) as usize;

                Beatrice20rc0_SetCodebook(
                    self.lib.phone_context,
                    self.lib.codebooks.as_ptr().add(offset),
                )
            }

            {
                let offset = new_target_speaker_id as usize
                    * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize;

                Beatrice20rc0_SetAdditiveSpeakerEmbedding(
                    self.lib.embedding_setter,
                    self.lib.additive_speaker_embeddings.as_ptr().add(offset),
                    self.lib.embedding_context,
                    self.lib.waveform_context,
                );
            }

            {
                let offset = new_target_speaker_id as usize
                    * (BEATRICE_20RC0_KV_LENGTH * BEATRICE_20RC0_KV_SPEAKER_EMBEDDING_CHANNELS)
                        as usize;

                Beatrice20rc0_RegisterKeyValueSpeakerEmbedding(
                    self.lib.embedding_setter,
                    self.lib.key_value_speaker_embeddings.as_ptr().add(offset),
                    self.lib.embedding_context,
                );
            }
        };

        self.info.target_speaker = new_target_speaker_id;
        self.lib.key_value_speaker_embedding_set_count = 0;

        Ok(())
    }

    fn set_formant_shift(&mut self, formant_shift: f64) {
        self.info.formant_shift = formant_shift;

        let index = (self.info.formant_shift * 2.0 + 4.0).round() as isize;

        debug_assert!((0..9).contains(&index));
        debug_assert!(
            self.lib.formant_shift_embeddings.len()
                == 9 * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as usize
        );

        unsafe {
            Beatrice20rc0_SetFormantShiftEmbedding(
                self.lib.embedding_setter,
                self.lib
                    .formant_shift_embeddings
                    .as_ptr()
                    .offset(index * BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS as isize),
                self.lib.embedding_context,
                self.lib.waveform_context,
            );
        }
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

    fn set_min_source_pitch(&mut self, min_source_pitch: f64) {
        self.info.min_source_pitch = min_source_pitch.clamp(0.0, 128.0);

        unsafe {
            Beatrice20rc0_SetMinQuantizedPitch(
                self.lib.pitch_context,
                (((self.info.min_source_pitch - 33.0)
                    * (BEATRICE_PITCH_BINS_PER_OCTAVE as f64 / 12.0)) as i32)
                    .clamp(1, BEATRICE_20RC0_PITCH_BINS as i32 - 1),
            )
        };
    }

    fn set_max_source_pitch(&mut self, max_source_pitch: f64) {
        self.info.max_source_pitch = max_source_pitch.clamp(0.0, 128.0);

        unsafe {
            Beatrice20rc0_SetMaxQuantizedPitch(
                self.lib.pitch_context,
                (((self.info.max_source_pitch - 33.0)
                    * (BEATRICE_PITCH_BINS_PER_OCTAVE as f64 / 12.0)) as i32)
                    .clamp(1, BEATRICE_20RC0_PITCH_BINS as i32 - 1),
            )
        };
    }

    fn set_vq_num_neighbors(&mut self, vq_num_neighbors: i32) {
        self.info.vq_num_neighbors = vq_num_neighbors.clamp(0, 8);

        unsafe {
            Beatrice20rc0_SetVQNumNeighbors(self.lib.phone_context, self.info.vq_num_neighbors);
        }
    }

    fn get_model_version(&self) -> &'static str {
        "2.0.0-rc.0"
    }
}
