use serde::Deserialize;
use std::{collections::HashMap, path::Path};

#[derive(Debug, Deserialize)]
pub struct BeatriceToml {
    pub model: ModelInfo,
    pub voice: HashMap<u32, Voice>,
}

impl BeatriceToml {
    pub fn load_from_tomlpath(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        let parsed: BeatriceToml = toml::from_str(&text)?;

        Ok(parsed)
    }
}

#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    pub version: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Voice {
    pub name: String,
    pub description: String,
    pub average_pitch: f64,

    #[serde(default)]
    pub portrait: Option<Portrait>,
}

#[derive(Debug, Deserialize)]
pub struct Portrait {
    pub path: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use crate::beatrice_toml::BeatriceToml;

    #[ignore]
    #[test]
    fn test_read() {
        let t = BeatriceToml::load_from_tomlpath("../test_file/beatrice.toml").unwrap();
        println!("{:#?}", t.model.version);
    }
}
