use std::path::Path;

use crate::{BeatriceBeta0, BeatriceBeta1, BeatriceRC0, BeatriceToml, errors::BeatriceError};

pub trait Beatrice {
    fn new(
        model_folder: impl AsRef<Path>,
        in_sample_rate: f64,
        out_sample_rate: f64,
        in_channel: u32,
        out_channel: u32,
    ) -> Result<Box<dyn Beatrice>, BeatriceError>
    where
        Self: Sized,
    {
        let model_folder = model_folder.as_ref();

        if !model_folder.is_dir() {
            return Err(BeatriceError::FileOpenError);
        }

        // tomlを探す
        let mut toml_path = None;
        for f in std::fs::read_dir(model_folder)?.flatten() {
            let path = f.path();
            if !path.is_file() {
                continue;
            }

            let Some(ext) = path.extension() else {
                continue;
            };

            if ext == "toml" {
                toml_path = Some(path);
                break;
            }
        }

        let Some(toml_path) = toml_path else {
            return Err(BeatriceError::FileOpenError);
        };

        let Ok(beatrice_toml) = BeatriceToml::load_from_modelpath(toml_path) else {
            return Err(BeatriceError::FileOpenError);
        };

        // それぞれのバージョンのbeatriceを読み込む
        let beatrice: Box<dyn Beatrice> = match beatrice_toml.model.version.as_str() {
            "2.0.0-rc.0" => {
                let mut beatrice_rc0 = Box::new(BeatriceRC0::new(
                    in_sample_rate,
                    out_sample_rate,
                    in_channel,
                    out_channel,
                ));

                beatrice_rc0.load_model(model_folder)?;
                beatrice_rc0
            }
            "2.0.0-beta.1" => {
                let mut beatrice_beta1 = Box::new(BeatriceBeta1::new(
                    in_sample_rate,
                    out_sample_rate,
                    in_channel,
                    out_channel,
                ));

                beatrice_beta1.load_model(model_folder)?;
                beatrice_beta1
            }
            v if v.starts_with("2.0.0-alpha") => {
                let mut beatrice_beta0 = Box::new(BeatriceBeta0::new(
                    in_sample_rate,
                    out_sample_rate,
                    in_channel,
                    out_channel,
                ));

                beatrice_beta0.load_model(model_folder)?;
                beatrice_beta0
            }

            _ => return Err(BeatriceError::FileOpenError),
        };

        Ok(beatrice)
    }

    fn infer(&mut self, input: &[f32]) -> Result<Vec<f32>, BeatriceError>;
    fn get_model_path(&self) -> Option<&Path>;
    fn get_n_speaker(&self) -> Option<i32>;
    fn set_target_speaker(&mut self, speaker: u32) -> Result<(), BeatriceError>;
    fn set_formant_shift(&mut self, formant_shift: f64);
    fn set_pitch_shift(&mut self, pitch_shift: f64);
    fn set_average_source_pitch(&mut self, average_source_pitch: f64);
    fn set_intonation_intensity(&mut self, intonation_intensity: f64);
    fn set_pitch_correction(&mut self, pitch_correction: f64);
    fn set_pitch_correction_type(&mut self, pitch_correction_type: i32);

    /* Only After RC.0 */
    fn set_min_source_pitch(&mut self, min_source_pitch: f64);
    fn set_max_source_pitch(&mut self, max_source_pitch: f64);
    fn set_vq_num_neighbors(&mut self, vq_num_neighbors: i32);
    fn get_model_version(&self) -> &'static str;
}
