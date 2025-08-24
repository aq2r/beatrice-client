use base64::prelude::*;
use std::fs;

#[tauri::command]
pub async fn other_read_image(path: String) -> Result<String, String> {
    match fs::read(path) {
        Ok(data) => {
            let base64_string = BASE64_STANDARD.encode(&data);
            Ok(format!("data:image/png;base64,{base64_string}"))
        }
        Err(_) => Err("".to_string()),
    }
}
