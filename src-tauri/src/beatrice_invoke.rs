use std::{
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
};

use anyhow::Context;
use beatrice_lib::{Beatrice, BeatriceError};
use serde::{Deserialize, Serialize};
use tauri::Runtime;
use tauri_plugin_dialog::DialogExt as _;
use tauri_plugin_store::StoreExt as _;

/* Voice Changer */

pub static BEATRICE: LazyLock<Mutex<Option<Beatrice>>> = LazyLock::new(|| Mutex::new(None));

#[tauri::command]
pub async fn beatrice_load_model(model_path: String) -> Result<(), String> {
    let mut beatrice = BEATRICE.lock().unwrap();

    match Beatrice::load_model(&model_path) {
        Ok(model) => *beatrice = Some(model),
        Err(err) => {
            *beatrice = None;
            return Err(err.to_string());
        }
    }

    Ok(())
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
pub async fn beatrice_set_pitch(pitch: f64) -> Result<(), String> {
    let mut beatrice = BEATRICE.lock().unwrap();

    let Some(beatrice) = beatrice.as_mut() else {
        return Err(BeatriceError::ModelNotLoaded.to_string());
    };

    beatrice.set_pitch_shift(pitch);
    Ok(())
}

#[tauri::command]
pub async fn beatrice_set_formant_shift(formant: f64) -> Result<(), String> {
    let mut beatrice = BEATRICE.lock().unwrap();

    let Some(beatrice) = beatrice.as_mut() else {
        return Err(BeatriceError::ModelNotLoaded.to_string());
    };

    beatrice.set_formant_shift(formant);
    Ok(())
}

#[tauri::command]
pub async fn beatrice_set_average_source_pitch(average_source_pitch: f64) -> Result<(), String> {
    let mut beatrice = BEATRICE.lock().unwrap();

    let Some(beatrice) = beatrice.as_mut() else {
        return Err(BeatriceError::ModelNotLoaded.to_string());
    };

    beatrice.set_average_source_pitch(average_source_pitch);
    Ok(())
}

#[tauri::command]
pub async fn beatrice_set_intonation_intensity(intonation_intensity: f64) -> Result<(), String> {
    let mut beatrice = BEATRICE.lock().unwrap();

    let Some(beatrice) = beatrice.as_mut() else {
        return Err(BeatriceError::ModelNotLoaded.to_string());
    };

    beatrice.set_intonation_intensity(intonation_intensity);
    Ok(())
}

#[tauri::command]
pub async fn beatrice_set_pitch_correction(pitch_correction: f64) -> Result<(), String> {
    let mut beatrice = BEATRICE.lock().unwrap();

    let Some(beatrice) = beatrice.as_mut() else {
        return Err(BeatriceError::ModelNotLoaded.to_string());
    };

    beatrice.set_pitch_correction(pitch_correction);
    Ok(())
}

#[tauri::command]
pub async fn beatrice_set_pitch_correction_type(pitch_correction_type: i32) -> Result<(), String> {
    let mut beatrice = BEATRICE.lock().unwrap();

    let Some(beatrice) = beatrice.as_mut() else {
        return Err(BeatriceError::ModelNotLoaded.to_string());
    };

    beatrice.set_pitch_correction_type(pitch_correction_type);
    Ok(())
}

/* model search */

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
    portrait_path: String,
    portrait_description: String,
}

const TAURI_STORE_MODEL_PATH_KEY: &str = "tauriStoreModelPath";
#[tauri::command]
pub async fn beatrice_search_model<R: Runtime>(
    app: tauri::AppHandle<R>,
    input_folder_path: Option<String>,
) -> Result<Vec<BeatriceModelInfo>, String> {
    let folder_path = {
        match input_folder_path {
            Some(val) => val,
            None => {
                let folder_path = app.dialog().file().blocking_pick_folder();
                let Some(folder_path) = folder_path else {
                    return Err("SelectCanceled".to_string());
                };

                let folder_path = folder_path.to_string();

                let store = app.store("store.json").map_err(|err| err.to_string())?;
                store.set(TAURI_STORE_MODEL_PATH_KEY, folder_path.clone());

                folder_path
            }
        }
    };

    // モデルがあるフォルダだけを集める
    let mut model_folders = vec![]; // (folder path, toml path)
    for entry in (std::fs::read_dir(folder_path).map_err(|err| err.to_string())?).flatten() {
        let folder_path = entry.path();
        if !folder_path.is_dir() {
            continue;
        }

        let model_files = [
            "formant_shift_embeddings.bin",
            "phone_extractor.bin",
            "pitch_estimator.bin",
            "speaker_embeddings.bin",
            "waveform_generator.bin",
        ];

        if !model_files
            .into_iter()
            .all(|file| folder_path.join(file).exists())
        {
            continue;
        }

        for file_entry in std::fs::read_dir(&folder_path)
            .map_err(|err| err.to_string())?
            .flatten()
        {
            let path = file_entry.path();
            if !path.is_file() {
                continue;
            }

            let Some(extention) = path.extension() else {
                continue;
            };

            if extention == "toml" {
                model_folders.push((folder_path, path));
                break;
            }
        }
    }

    let mut model_info = vec![];
    for (model_path, toml_path) in model_folders.iter() {
        let toml_str = std::fs::read_to_string(toml_path).map_err(|err| err.to_string())?;

        if let Ok(toml_data) = parse_model_toml(model_path, &toml_str) {
            model_info.push(toml_data);
        }
    }

    Ok(model_info)
}

fn parse_model_toml(model_path: &Path, data: &str) -> anyhow::Result<BeatriceModelInfo> {
    let value: toml::Value = toml::from_str(data)?;

    let Some(model) = value.get("model") else {
        anyhow::bail!("key: 'model' not found");
    };

    let model_get_to_text = |key: &str| -> anyhow::Result<String> {
        Ok(model
            .get(key)
            .context(format!("Key: '{key}' not found"))?
            .as_str()
            .unwrap_or("")
            .to_string())
    };

    let version = model_get_to_text("version")?;
    let name = model_get_to_text("name")?;
    let description = model_get_to_text("description")?;

    let Some(voice_table) = value.get("voice") else {
        anyhow::bail!("key: 'voice' not found");
    };

    let mut voices = vec![];
    for i in 0.. {
        if let Some(voice) = voice_table.get(i.to_string()) {
            let voice_get_to_text = |key: &str| -> anyhow::Result<String> {
                Ok(voice
                    .get(key)
                    .context(format!("Key: '{key}' not found"))?
                    .as_str()
                    .unwrap_or("")
                    .to_string())
            };

            let voice_name = voice_get_to_text("name")?;
            let voice_description = voice_get_to_text("description")?;

            let portrait_table = voice.get("portrait").context("Key: 'portrait' not found")?;
            let portrait_get_to_text = |key: &str| -> anyhow::Result<String> {
                Ok(portrait_table
                    .get(key)
                    .context(format!("Key: '{key}' not found"))?
                    .as_str()
                    .unwrap_or("")
                    .to_string())
            };

            let portrait_path = portrait_get_to_text("path")?;
            let portrait_description = portrait_get_to_text("description")?;

            let average_pitch = voice
                .get("average_pitch")
                .context("Key: 'average_pitch' not found")?
                .as_float()
                .unwrap_or(0.0);

            voices.push(BeatriceVoiceInfo {
                name: voice_name,
                description: voice_description,
                average_pitch: average_pitch as f32,
                portrait_path,
                portrait_description,
            });
        } else {
            break;
        }
    }

    Ok(BeatriceModelInfo {
        model_path: model_path.to_path_buf(),
        version,
        name,
        description,
        voices,
    })
}
