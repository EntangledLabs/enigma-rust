use dotenv::dotenv;
use std::{
    env,
    fs,
    process::exit
};
use serde::Deserialize;
use log::{info, warn, error};

use crate::db;

// struct Config stores all of the configuration options
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub db_pw: String,
    pub cfg: TomlConfig
}

impl Config {
    fn new(api_key: String, db_pw: String, cfg: TomlConfig) -> Config {
        Config {
            api_key: api_key,
            db_pw: db_pw,
            cfg: cfg,
        }
    }
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct TomlConfig {
    pub general: General,
    pub database: Database,
    pub environment: Environment,
    pub boxes: Boxes,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct General {
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub database: String
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Environment {

}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Boxes {

}

// fn read_config reads an api key and db pw from the environment and the rest of the config from a configuration file 'config.toml'
pub fn read_config() -> Config {
    dotenv().ok();

    info!("Beginning config read");
    
    let api_key: String = match env::var("API_KEY") {
        Ok(val) => val,
        Err(e) => {
            error!("API Key not found. Terminating...");
            exit(1);
        }
    };

    let db_pw: String = match env::var("POSTGRES_PASSWORD") {
        Ok(val) => val,
        Err(e) => {
            error!("Password for DB not found. Terminating...");
            exit(1);
        }
    };

    let cfg_filename = "config.toml";
    let contents = match fs::read_to_string(cfg_filename) {
        Ok(c) => c,
        Err(_) => {
            error!("Could not read '{}'", cfg_filename);
            exit(1);
        }
    };

    let config: TomlConfig = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            error!("Unable to load configuration data from '{}'", cfg_filename);
            exit(1);
        }
    };

    info!("Config read, values imported");

    Config::new(api_key, db_pw, config)

}