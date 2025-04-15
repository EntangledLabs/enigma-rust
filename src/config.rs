use dotenv::dotenv;
use std::{
    env,
    fs,
    process::exit
};
use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
    general: General,
    database: Database,
    environment: Environment,
    boxes: Boxes,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct General {
    api_key: String,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Database {
    host: String,
    port: u16,
    username: String,
    password: String,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Environment {

}

#[derive(Deserialize)]
#[derive(Debug)]
struct Boxes {

}

pub fn read_config() -> Config {
    dotenv().ok();

    println!("Beginning config read");
  
    let cfg_filename = "config.toml";
    let contents = match fs::read_to_string(cfg_filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read '{}'", cfg_filename);
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load configuration data from '{}'", cfg_filename);
            exit(1);
        }
    };

    println!("Config read, values imported");

    config
}