use postgres::{
    Client,
    Error,
    NoTls
};

use std::{
    fs::read,
    fmt,
    collections::HashMap
};
use crate::config;

fn create_tables() -> Result<(), Error>{
    let mut client = Client::connect(
        format!("")
    );

    Ok(())
}