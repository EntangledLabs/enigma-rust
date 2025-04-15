use tokio_postgres::{
    Error,
    NoTls,
    Client
};

use std::{
    fmt,
    collections::HashMap,
    process::exit
};

use log::{info, warn, error};

use crate::config::Database;

#[tokio::main]
pub async fn connect_db(cfg: &Database, db_pw: &str) -> Client {
    let (client, connection) =
        tokio_postgres::connect(
            format!(
                "host={host} port={port} user={user} password={pw} dbname={db}",
                host = cfg.host,
                port = cfg.port,
                user = cfg.username,
                pw = db_pw,
                db = cfg.database
            ).as_str(),
            NoTls
        )
        .await
        .unwrap();
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Database connection failed. Terminating...");
            exit(1);
        }
    });

    info!("Database connection successful");

    client
}