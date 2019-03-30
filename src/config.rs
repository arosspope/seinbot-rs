use log::info;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::error::Error;
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
        let mut file = File::open(path_file).expect("failed to open file");

        serde_json::from_reader(&mut file).ok().unwrap()
    }

    pub fn from_env() -> Result<Config, Box<dyn Error>> {
        info!("loading secrets from system environment");

        let consumer_key = env::var("SEINBOT_CONSUMER_KEY")?;
        let consumer_secret = env::var("SEINBOT_CONSUMER_SECRET")?;
        let access_key = env::var("SEINBOT_ACCESS_KEY")?;
        let access_secret = env::var("SEINBOT_ACCESS_SECRET")?;

        Ok(Config {
            consumer_key,
            consumer_secret,
            access_key,
            access_secret,
        })
    }
}
