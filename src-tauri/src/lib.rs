mod beatrice;
mod cpal;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            cpal::cpal_get_inputs,
            cpal::cpal_get_outputs,
            cpal::cpal_start_voice_changer,
            beatrice::beatrice_load_model,
            beatrice::beatrice_set_pitch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
