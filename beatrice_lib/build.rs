use std::{fs::File, path::PathBuf};

use reqwest::blocking::Client;

fn main() {
    // beatrice のライブラリをダウンロード
    let lib_path = PathBuf::from("lib/beatrice.lib");

    if !lib_path.exists() {
        let lib_url = "https://huggingface.co/fierce-cats/beatrice-2.0.0-alpha/resolve/rc.0/rc.0/beatrice.lib";
        let response = Client::new()
            .get(lib_url)
            .send()
            .expect("Failed download lib");

        let mut file = File::create("lib/beatrice.lib").expect("Failed create File");
        let content = response.bytes().expect("Failed convert");
        std::io::copy(&mut content.as_ref(), &mut file).expect("Failed copy");
    }

    // beatriceライブラリをリンク
    println!("cargo:rustc-link-search=native=beatrice_lib/lib");
    println!("cargo:rustc-link-lib=static=beatrice");
}
