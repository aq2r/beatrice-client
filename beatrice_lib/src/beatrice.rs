use std::path::Path;

use crate::{
    BeatriceError, beatrice_beta_0::BeatriceBeta0, beatrice_beta_1::BeatriceBeta1,
    beatrice_rc_0::BeatriceRC0,
};

macro_rules! delegate_method {
    ($self:expr, $method:ident($($arg:expr),*)) => {
        match $self {
            Beatrice::BeatriceBeta0(b) => b.$method($($arg),*),
            Beatrice::BeatriceBeta1(b) => b.$method($($arg),*),
            Beatrice::BeatriceRC0(b) =>  b.$method($($arg),*),
        }
    };
}

pub enum Beatrice {
    BeatriceBeta0(Box<BeatriceBeta0>),
    BeatriceBeta1(Box<BeatriceBeta1>),
    BeatriceRC0(Box<BeatriceRC0>),
}

impl Beatrice {
    pub fn load_model(model_path: impl AsRef<Path>) -> Result<Beatrice, BeatriceError> {
        let model_version = Self::internal_get_model_version(&model_path)?;

        let mut beatrice = match model_version.as_str() {
            "2.0.0-rc.0" => Beatrice::BeatriceRC0(BeatriceRC0::new().into()),
            "2.0.0-beta.1" => Beatrice::BeatriceBeta1(BeatriceBeta1::new().into()),
            v if v.starts_with("2.0.0-alpha") => {
                Beatrice::BeatriceBeta0(BeatriceBeta0::new().into())
            }

            _ => return Err(BeatriceError::FileOpenError),
        };

        delegate_method!(&mut beatrice, load_model(&model_path))?;
        Ok(beatrice)
    }

    pub fn infer(&mut self, input: &[f32]) -> Result<Vec<f32>, BeatriceError> {
        delegate_method!(self, infer(input))
    }

    pub fn set_input_setting(
        &mut self,
        sample_rate: f64,
        channel: usize,
    ) -> Result<(), rubato::ResamplerConstructionError> {
        delegate_method!(self, set_input_setting(sample_rate, channel))
    }

    pub fn set_output_setting(
        &mut self,
        sample_rate: f64,
        channel: usize,
    ) -> Result<(), rubato::ResamplerConstructionError> {
        delegate_method!(self, set_output_setting(sample_rate, channel))
    }

    pub fn get_n_speaker(&self) -> Option<i32> {
        delegate_method!(self, get_n_speaker())
    }

    pub fn set_target_speaker(&mut self, speaker: u32) -> Result<(), BeatriceError> {
        delegate_method!(self, set_target_speaker(speaker))
    }

    pub fn set_formant_shift(&mut self, formant_shift: f64) {
        delegate_method!(self, set_formant_shift(formant_shift))
    }

    pub fn set_pitch_shift(&mut self, pitch_shift: f64) {
        delegate_method!(self, set_pitch_shift(pitch_shift))
    }

    pub fn set_average_source_pitch(&mut self, average_source_pitch: f64) {
        delegate_method!(self, set_average_source_pitch(average_source_pitch))
    }

    pub fn set_intonation_intensity(&mut self, intonation_intensity: f64) {
        delegate_method!(self, set_intonation_intensity(intonation_intensity))
    }

    pub fn set_pitch_correction(&mut self, pitch_correction: f64) {
        delegate_method!(self, set_pitch_correction(pitch_correction))
    }

    pub fn set_pitch_correction_type(&mut self, pitch_correction_type: i32) {
        delegate_method!(self, set_pitch_correction_type(pitch_correction_type))
    }

    pub fn set_min_source_pitch(&mut self, min_source_pitch: f64) {
        if let Beatrice::BeatriceRC0(b) = self {
            b.set_min_source_pitch(min_source_pitch)
        }
    }

    pub fn set_max_source_pitch(&mut self, max_source_pitch: f64) {
        if let Beatrice::BeatriceRC0(b) = self {
            b.set_max_source_pitch(max_source_pitch)
        }
    }

    pub fn set_vq_num_neighbors(&mut self, vq_num_neighbors: i32) {
        if let Beatrice::BeatriceRC0(b) = self {
            b.set_vq_num_neighbors(vq_num_neighbors)
        }
    }

    pub fn get_model_version(&self) -> String {
        match self {
            Beatrice::BeatriceBeta0(_) => "2.0.0-alpha".into(),
            Beatrice::BeatriceBeta1(_) => "2.0.0-beta.1".into(),
            Beatrice::BeatriceRC0(_) => "2.0.0-rc.0".into(),
        }
    }

    fn internal_get_model_version(model_path: impl AsRef<Path>) -> Result<String, BeatriceError> {
        // search toml
        let mut toml_path = None;
        for file_entry in std::fs::read_dir(&model_path)?.flatten() {
            let path = file_entry.path();
            if !path.is_file() {
                continue;
            }

            let Some(extention) = path.extension() else {
                continue;
            };

            if extention == "toml" {
                toml_path = Some(path);
                break;
            }
        }

        let Some(toml_path) = toml_path else {
            return Err(BeatriceError::FileOpenError);
        };

        // get model version
        let toml_str = std::fs::read_to_string(toml_path)?;
        let value: toml::Value = toml::from_str(&toml_str)?;
        let Some(version) = value.get("model").and_then(|v| v.get("version")) else {
            return Err(BeatriceError::FileOpenError);
        };

        Ok(version.as_str().unwrap_or("None").to_string())
    }
}
