use std::path::PathBuf;

use beatrice_lib::{BeatriceError, BeatriceToml};
use serde::{Deserialize, Serialize};

use crate::cpal_invoke::BEATRICE;

#[derive(Debug, Serialize, Deserialize)]
pub struct BeatriceModelInfo {
    model_path: PathBuf,
    version: String,
    name: String,
    description: String,

    voices: Vec<BeatriceVoiceInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeatriceVoiceInfo {
    name: String,
    description: String,
    average_pitch: f32,
    portrait_path: Option<String>,
    portrait_description: Option<String>,
}

#[tauri::command]
pub async fn beatrice_get_model_from_path(model_folder: String) -> Option<BeatriceModelInfo> {
    let model_folder_path = PathBuf::from(model_folder);

    let beta_model_files = [
        "formant_shift_embeddings.bin",
        "phone_extractor.bin",
        "pitch_estimator.bin",
        "speaker_embeddings.bin",
        "waveform_generator.bin",
    ];

    let rc_model_files = [
        "speaker_embeddings.bin",
        "waveform_generator.bin",
        "embedding_setter.bin",
        "pitch_estimator.bin",
    ];

    let is_beta_files_exists = beta_model_files
        .into_iter()
        .all(|f| model_folder_path.join(f).exists());

    let is_rc_files_exists = rc_model_files
        .into_iter()
        .all(|f| model_folder_path.join(f).exists());

    if !(is_beta_files_exists || is_rc_files_exists) {
        return None;
    }

    let mut toml_path = None;
    for f in std::fs::read_dir(&model_folder_path).ok()?.flatten() {
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
        return None;
    };

    let beatrice_toml = BeatriceToml::load_from_tomlpath(toml_path).ok()?;

    let mut beatrice_voice_info = vec![];
    for i in 0.. {
        let Some(voice) = beatrice_toml.voice.get(&i) else {
            break;
        };

        let voice_info = BeatriceVoiceInfo {
            name: voice.name.clone(),
            description: voice.description.clone(),
            average_pitch: voice.average_pitch as f32,
            portrait_path: voice.portrait.as_ref().map(|i| i.path.to_string()),
            portrait_description: voice.portrait.as_ref().map(|i| i.description.clone()),
        };

        beatrice_voice_info.push(voice_info);
    }

    Some(BeatriceModelInfo {
        model_path: model_folder_path,
        version: beatrice_toml.model.version,
        name: beatrice_toml.model.name,
        description: beatrice_toml.model.description,
        voices: beatrice_voice_info,
    })
}

#[tauri::command]
pub async fn beatrice_get_nspeaker() -> Option<i32> {
    let beatrice = BEATRICE.lock().unwrap();
    let beatrice = beatrice.as_ref()?;

    beatrice.get_n_speaker()
}

#[tauri::command]
pub async fn beatrice_set_target_speaker(target: i32) -> Result<(), String> {
    let mut beatrice = BEATRICE.lock().unwrap();

    let Some(beatrice) = beatrice.as_mut() else {
        return Err(BeatriceError::ModelNotLoaded.to_string());
    };

    beatrice
        .set_target_speaker(target as u32)
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn beatrice_get_version() -> Option<String> {
    let mut beatrice = BEATRICE.lock().unwrap();

    Some(beatrice.as_mut()?.get_model_version().to_string())
}

macro_rules! beatrice_command {
    (
        $fn_name:ident,
        $method:ident,
        $arg:ident : $ty:ty
    ) => {
        #[tauri::command]
        pub async fn $fn_name($arg: $ty) -> Result<(), String> {
            let mut beatrice = BEATRICE.lock().unwrap();

            let Some(beatrice) = beatrice.as_mut() else {
                return Err(BeatriceError::ModelNotLoaded.to_string());
            };

            beatrice.$method($arg);
            Ok(())
        }
    };
}

beatrice_command!(
    beatrice_set_pitch,
    set_pitch_shift,
    pitch: f64
);

beatrice_command!(
    beatrice_set_formant_shift,
    set_formant_shift,
    formant: f64
);

beatrice_command!(
    beatrice_set_average_source_pitch,
    set_average_source_pitch,
    average_source_pitch: f64
);

beatrice_command!(
    beatrice_set_intonation_intensity,
    set_intonation_intensity,
    intonation_intensity: f64
);

beatrice_command!(
    beatrice_set_min_source_pitch,
    set_min_source_pitch,
    min_source_pitch: f64
);

beatrice_command!(
    beatrice_set_max_source_pitch,
    set_max_source_pitch,
    max_source_pitch: f64
);

beatrice_command!(
    beatrice_set_vq_num_neighbors,
    set_vq_num_neighbors,
    vq_num_neighbors: i32
);
