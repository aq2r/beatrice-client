#![allow(non_camel_case_types)]
#![allow(clippy::enum_variant_names)]
#![allow(unused)]

use std::ffi::{c_char, c_float, c_int, c_void};

pub const BEATRICE_IN_HOP_LENGTH: usize = 160;
pub const BEATRICE_OUT_HOP_LENGTH: usize = 240;
pub const BEATRICE_PHONE_CHANNELS: usize = 256;
pub const BEATRICE_PITCH_BINS: usize = 384;
pub const BEATRICE_PITCH_BINS_PER_OCTAVE: usize = 96;
pub const BEATRICE_WAVEFORM_GENERATOR_HIDDEN_CHANNELS: usize = 256;
pub const BEATRICE_IN_SAMPLE_RATE: usize = 16000;
pub const BEATRICE_OUT_SAMPLE_RATE: usize = 24000;

#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Beatrice_ErrorCode {
    Beatrice_kSuccess = 0,
    Beatrice_kFileOpenError,
    Beatrice_kFileTooSmall,
    Beatrice_kFileTooLarge,
    Beatrice_kInvalidFileSize,
}

/* -------- 20a2 --------  */

#[repr(C)]
pub struct Beatrice20a2_PhoneExtractor(*const c_void);

#[repr(C)]
pub struct Beatrice20a2_PhoneContext1(*const c_void);

#[repr(C)]
pub struct Beatrice20a2_PitchEstimator(*const c_void);

#[repr(C)]
pub struct Beatrice20a2_PitchContext1(*const c_void);

#[repr(C)]
pub struct Beatrice20a2_WaveformGenerator(*const c_void);

#[repr(C)]
pub struct Beatrice20a2_WaveformContext1(*const c_void);

#[link(name = "beatrice", kind = "static")]
unsafe extern "C" {
    // Phone Extractor
    pub fn Beatrice20a2_CreatePhoneExtractor() -> *mut Beatrice20a2_PhoneExtractor;
    pub fn Beatrice20a2_DestroyPhoneExtractor(phone_extractor: *mut Beatrice20a2_PhoneExtractor);

    pub fn Beatrice20a2_CreatePhoneContext1() -> *mut Beatrice20a2_PhoneContext1;
    pub fn Beatrice20a2_DestroyPhoneContext1(ctx: *mut Beatrice20a2_PhoneContext1);

    pub fn Beatrice20a2_ReadPhoneExtractorParameters(
        phone_extractor: *mut Beatrice20a2_PhoneExtractor,
        filename: *const c_char,
    ) -> Beatrice_ErrorCode;

    pub fn Beatrice20a2_ExtractPhone1(
        phone_extractor: *const Beatrice20a2_PhoneExtractor,
        input: *const c_float,
        output: *mut c_float,
        ctx: *mut Beatrice20a2_PhoneContext1,
    );

    // Pitch Estimator
    pub fn Beatrice20a2_CreatePitchEstimator() -> *mut Beatrice20a2_PitchEstimator;
    pub fn Beatrice20a2_DestroyPitchEstimator(pitch_estimator: *mut Beatrice20a2_PitchEstimator);

    pub fn Beatrice20a2_CreatePitchContext1() -> *mut Beatrice20a2_PitchContext1;
    pub fn Beatrice20a2_DestroyPitchContext1(ctx: *mut Beatrice20a2_PitchContext1);

    pub fn Beatrice20a2_ReadPitchEstimatorParameters(
        pitch_estimator: *mut Beatrice20a2_PitchEstimator,
        filename: *const c_char,
    ) -> Beatrice_ErrorCode;

    pub fn Beatrice20a2_EstimatePitch1(
        pitch_estimator: *const Beatrice20a2_PitchEstimator,
        input: *const c_float,
        output_quantized_pitch: *mut c_int,
        output_pitch_feature: *mut c_float,
        ctx: *mut Beatrice20a2_PitchContext1,
    );

    pub fn Beatrice20a2_ReadNSpeakers(
        file_name: *const c_char,
        output: *mut c_int,
    ) -> Beatrice_ErrorCode;
    pub fn Beatrice20a2_ReadSpeakerEmbeddings(
        file_name: *const c_char,
        output: *mut c_float,
    ) -> Beatrice_ErrorCode;

    // Waveform Generator
    pub fn Beatrice20a2_CreateWaveformGenerator() -> *mut Beatrice20a2_WaveformGenerator;
    pub fn Beatrice20a2_DestroyWaveformGenerator(
        waveform_generator: *mut Beatrice20a2_WaveformGenerator,
    );

    pub fn Beatrice20a2_CreateWaveformContext1() -> *mut Beatrice20a2_WaveformContext1;
    pub fn Beatrice20a2_DestroyWaveformContext1(ctx: *mut Beatrice20a2_WaveformContext1);

    pub fn Beatrice20a2_ReadWaveformGeneratorParameters(
        waveform_generator: *mut Beatrice20a2_WaveformGenerator,
        file_name: *const c_char,
    ) -> Beatrice_ErrorCode;

    pub fn Beatrice20a2_GenerateWaveform1(
        waveform_generator: *const Beatrice20a2_WaveformGenerator,
        input_phone: *const c_float,
        input_quantized_pitch: *const c_int,
        input_pitch_features: *const c_float,
        input_speaker_embedding: *const c_float,
        output: *mut c_float,
        ctx: *mut Beatrice20a2_WaveformContext1,
    );
}

/* -------- 20b1 --------  */

#[repr(C)]
pub struct Beatrice20b1_PhoneExtractor(*const c_void);

#[repr(C)]
pub struct Beatrice20b1_PhoneContext1(*const c_void);

#[repr(C)]
pub struct Beatrice20b1_PitchEstimator(*const c_void);

#[repr(C)]
pub struct Beatrice20b1_PitchContext1(*const c_void);

#[repr(C)]
pub struct Beatrice20b1_WaveformGenerator(*const c_void);

#[repr(C)]
pub struct Beatrice20b1_WaveformContext1(*const c_void);

#[link(name = "beatrice", kind = "static")]
unsafe extern "C" {
    // Phone Extractor
    pub fn Beatrice20b1_CreatePhoneExtractor() -> *mut Beatrice20b1_PhoneExtractor;
    pub fn Beatrice20b1_DestroyPhoneExtractor(phone_extractor: *mut Beatrice20b1_PhoneExtractor);

    pub fn Beatrice20b1_CreatePhoneContext1() -> *mut Beatrice20b1_PhoneContext1;
    pub fn Beatrice20b1_DestroyPhoneContext1(ctx: *mut Beatrice20b1_PhoneContext1);

    pub fn Beatrice20b1_ReadPhoneExtractorParameters(
        phone_extractor: *mut Beatrice20b1_PhoneExtractor,
        file_name: *const c_char,
    ) -> Beatrice_ErrorCode;

    pub fn Beatrice20b1_ExtractPhone1(
        phone_extractor: *const Beatrice20b1_PhoneExtractor,
        input: *const c_float,
        output: *mut c_float,
        ctx: *mut Beatrice20b1_PhoneContext1,
    );

    // Pitch Estimator
    pub fn Beatrice20b1_CreatePitchEstimator() -> *mut Beatrice20b1_PitchEstimator;
    pub fn Beatrice20b1_DestroyPitchEstimator(pitch_estimator: *mut Beatrice20b1_PitchEstimator);

    pub fn Beatrice20b1_CreatePitchContext1() -> *mut Beatrice20b1_PitchContext1;
    pub fn Beatrice20b1_DestroyPitchContext1(ctx: *mut Beatrice20b1_PitchContext1);

    pub fn Beatrice20b1_ReadPitchEstimatorParameters(
        pitch_estimator: *mut Beatrice20b1_PitchEstimator,
        file_name: *const c_char,
    ) -> Beatrice_ErrorCode;

    pub fn Beatrice20b1_EstimatePitch1(
        pitch_estimator: *const Beatrice20b1_PitchEstimator,
        input: *const c_float,
        output_quantized_pitch: *mut c_int,
        output_pitch_feature: *mut c_float,
        ctx: *mut Beatrice20b1_PitchContext1,
    );

    // Speaker Embeddings
    pub fn Beatrice20b1_ReadNSpeakers(
        file_name: *const c_char,
        output: *mut c_int,
    ) -> Beatrice_ErrorCode;

    pub fn Beatrice20b1_ReadSpeakerEmbeddings(
        file_name: *const c_char,
        output: *mut c_float,
    ) -> Beatrice_ErrorCode;

    // Waveform Generator
    pub fn Beatrice20b1_CreateWaveformGenerator() -> *mut Beatrice20b1_WaveformGenerator;
    pub fn Beatrice20b1_DestroyWaveformGenerator(
        waveform_generator: *mut Beatrice20b1_WaveformGenerator,
    );

    pub fn Beatrice20b1_CreateWaveformContext1() -> *mut Beatrice20b1_WaveformContext1;
    pub fn Beatrice20b1_DestroyWaveformContext1(ctx: *mut Beatrice20b1_WaveformContext1);

    pub fn Beatrice20b1_ReadWaveformGeneratorParameters(
        waveform_generator: *mut Beatrice20b1_WaveformGenerator,
        file_name: *const c_char,
    ) -> Beatrice_ErrorCode;

    pub fn Beatrice20b1_GenerateWaveform1(
        waveform_generator: *const Beatrice20b1_WaveformGenerator,
        input_phone: *const c_float,
        input_quantized_pitch: *const c_int,
        input_pitch_features: *const c_float,
        input_speaker_embedding: *const c_float,
        output: *mut c_float,
        ctx: *mut Beatrice20b1_WaveformContext1,
    );
}
