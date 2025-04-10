use toml::Table;
use dotenv::dotenv;
use std::env;

fn read_config() {
    dotenv().ok();

    println!("Beginning config read");

    let api_key = env::var("API_KEY");

    match api_key {
        Ok(val) => println!("API_KEY: {:?}", val),
        Err(e) => println!("Invalid API_KEY: {}", e)
    }

    println!("Config read, values imported");
}