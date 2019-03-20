use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
}

impl Config {
    pub fn read(path_file: &Path) -> Config {
        let file = File::open(path_file);

        if file.is_err() {
            return Config::from_env(); // Try loading from env as a fallback
        }

        serde_json::from_reader(&mut file.unwrap())
            .ok()
            .unwrap_or_else(Config::from_env) // Again as a fallback
    }

    /// This should only be called as a fallback
    pub fn from_env() -> Config {
        let consumer_key = match env::var("SEINBOT_CONSUMER_KEY") {
            Ok(val) => val,
            Err(_) => panic!("couldn't find consumer key"),
        };

        let consumer_secret = match env::var("SEINBOT_CONSUMER_SECRET") {
            Ok(val) => val,
            Err(_) => panic!("couldn't find consumer secret"),
        };

        let access_key = match env::var("SEINBOT_ACCESS_KEY") {
            Ok(val) => val,
            Err(_) => panic!("couldn't find access key"),
        };

        let access_secret = match env::var("SEINBOT_ACCESS_SECRET") {
            Ok(val) => val,
            Err(_) => panic!("couldn't find access secret"),
        };

        Config {
            consumer_key,
            consumer_secret,
            access_key,
            access_secret,
        }
    }
}
