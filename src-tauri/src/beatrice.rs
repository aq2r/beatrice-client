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
pub async fn beatrice_set_pitch(pitch: f64) {
    let mut beatrice = BEATRICE.lock().unwrap();
    beatrice.set_pitch_shift(pitch);
}
