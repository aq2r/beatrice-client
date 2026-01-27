mod beatrice_invoke;
mod cpal_invoke;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            cpal_invoke::cpal_get_inputs,
            cpal_invoke::cpal_get_outputs,
            cpal_invoke::cpal_set_input_gain,
            cpal_invoke::cpal_set_output_gain,
            cpal_invoke::cpal_set_monitor_gain,
            cpal_invoke::cpal_start_voice_changer,
            cpal_invoke::cpal_set_input_threshold,
            beatrice_invoke::beatrice_get_model_from_path,
            beatrice_invoke::beatrice_get_nspeaker,
            beatrice_invoke::beatrice_set_target_speaker,
            beatrice_invoke::beatrice_get_version,
            beatrice_invoke::beatrice_set_pitch,
            beatrice_invoke::beatrice_set_formant_shift,
            beatrice_invoke::beatrice_set_average_source_pitch,
            beatrice_invoke::beatrice_set_min_source_pitch,
            beatrice_invoke::beatrice_set_max_source_pitch,
            beatrice_invoke::beatrice_set_vq_num_neighbors,
            beatrice_invoke::beatrice_set_intonation_intensity,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
