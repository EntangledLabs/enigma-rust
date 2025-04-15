use log::{info, warn, debug, error};
use colog;

use tokio_postgres::Client;

mod config;
mod db;

fn main() {
    colog::init();
    info!("Starting Engima Scoring Engine");
    
    // grabs config from env and config.toml
    info!("Checking for valid configuration options...");
    let cfg: config::Config = config::read_config();

    // checks if postgres, rabbitmq are running
    info!("Checking for essential services...");
    let client: Client = db::connect_db(&cfg.cfg.database, &cfg.db_pw);

    // startup complete, listening for requests
    info!("Startup complete! Engima is up and running.");
    info!("Listening...");
    println!("{:?}", cfg);
}
