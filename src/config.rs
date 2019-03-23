use log::info;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use simple_error::bail;
use std::fmt;

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

        serde_json::from_reader(&mut file)
            .ok()
            .unwrap()
    }

    pub fn from_env() -> Result<Config, Box<Error>> {
        info!("loading secrets from system environment");
        let consumer_key = match env::var("SEINBOT_CONSUMER_KEY") {
            Ok(val) => val,
            Err(_) => bail!("couldn't find consumer key"),
        };

        let consumer_secret = match env::var("SEINBOT_CONSUMER_SECRET") {
            Ok(val) => val,
            Err(_) => bail!("couldn't find consumer secret"),
        };

        let access_key = match env::var("SEINBOT_ACCESS_KEY") {
            Ok(val) => val,
            Err(_) => bail!("couldn't find access key"),
        };

        let access_secret = match env::var("SEINBOT_ACCESS_SECRET") {
            Ok(val) => val,
            Err(_) => bail!("couldn't find access secret"),
        };

        Ok(Config {
            consumer_key,
            consumer_secret,
            access_key,
            access_secret,
        })
    }
}
