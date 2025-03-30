use std::{
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use beatrice_lib::Beatrice;

pub static BEATRICE: LazyLock<Mutex<Beatrice>> = LazyLock::new(|| Mutex::new(Beatrice::new()));

#[tauri::command]
pub async fn beatrice_load_model(model_path: String) -> Result<(), String> {
    let mut beatrice = BEATRICE.lock().unwrap();

    beatrice.reset_context();
    if let Err(err) = beatrice.load_model(&model_path) {
        return Err(err.to_string());
    };

    Ok(())
}

#[tauri::command]
pub async fn beatrice_get_nspeaker() -> Option<i32> {
    let beatrice = BEATRICE.lock().unwrap();
    beatrice.get_n_speaker()
}

#[tauri::command]
pub async fn beatrice_set_target_speaker(target: i32) -> Result<(), String> {
    let mut beatrice = BEATRICE.lock().unwrap();
    beatrice
        .set_target_speaker(target as u32)
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn beatrice_set_pitch(pitch: f64) {
    let mut beatrice = BEATRICE.lock().unwrap();
    beatrice.set_pitch_shift(pitch);
}

#[tauri::command]
pub async fn beatrice_set_formant_shift(formant: f64) {
    let mut beatrice = BEATRICE.lock().unwrap();
    beatrice.set_formant_shift(formant);
}

#[tauri::command]
pub async fn beatrice_set_average_source_pitch(average_source_pitch: f64) {
    let mut beatrice = BEATRICE.lock().unwrap();
    beatrice.set_average_source_pitch(average_source_pitch);
}

#[tauri::command]
pub async fn beatrice_set_intonation_intensity(intonation_intensity: f64) {
    let mut beatrice = BEATRICE.lock().unwrap();
    beatrice.set_intonation_intensity(intonation_intensity);
}

#[tauri::command]
pub async fn beatrice_set_pitch_correction(pitch_correction: f64) {
    let mut beatrice = BEATRICE.lock().unwrap();
    beatrice.set_pitch_correction(pitch_correction);
}

#[tauri::command]
pub async fn beatrice_set_pitch_correction_type(pitch_correction_type: i32) {
    let mut beatrice = BEATRICE.lock().unwrap();
    beatrice.set_pitch_correction_type(pitch_correction_type);
}
