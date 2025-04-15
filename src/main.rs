mod config;
mod db;

fn main() {
    let cfg: config::Config = config::read_config();

    println!("{:?}", cfg);
}
