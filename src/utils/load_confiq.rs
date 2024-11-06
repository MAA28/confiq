use std::path::PathBuf;

use log::info;

use crate::confiq::Confiq;

impl Confiq {
    pub fn load(path: &PathBuf) -> Confiq {
        let text = std::fs::read_to_string(&path)
            .expect(format!("Could not find file at {}", path.display()).as_str());
        info!("Sourcing from {}", path.display());
        toml::from_str(&text)
            .expect(format!("Could not source file at {}", path.display()).as_str())
    }
}
