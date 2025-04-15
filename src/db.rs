use tokio_postgres::{
    Error,
    NoTls,
    Client
};

use std::{
    fmt,
    collections::HashMap
};

#[tokio::main]
async fn connect_db(host: &str, port: u16, user: &str, password: &str) -> (Result<(), Error>, Client) {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls)
        .await
        .unwrap();
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    (Ok(()), client)
}