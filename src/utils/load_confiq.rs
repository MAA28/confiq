use std::path::PathBuf;

use log::{info, warn};
use dirs::config_dir;

use crate::confiq::Confiq;

impl Confiq {
    pub fn load(path: Option<PathBuf>) -> Confiq {
        match path {
            Some(path) => {
                let text = std::fs::read_to_string(&path)
                    .expect(format!("Could not find file at {}", path.display()).as_str());
                info!("Sourcing from {}", path.display());
                toml::from_str(&text)
                    .expect(format!("Could not source file at {}", path.display()).as_str())
            }
            None => {
                warn!("No config path specified. Using default paths");

                let path = config_dir()
                    .expect("Could not find config dir")
                    .join("confiq.toml");

                let text = std::fs::read_to_string(&path)
                    .expect(format!("Could not find file at {}", path.display()).as_str());
                info!("Sourcing from {}", path.display());
                toml::from_str(&text)
                    .expect(format!("Could not source file at {}", path.display()).as_str())
            }
        }
    }
}
