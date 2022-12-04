use serde_derive::Deserialize;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    // let result = reqwest::get("http://192.168.10.196/api/show/text/HiBaby").await;
    // println!("{:#?}", result)
    read_toml_config()
}

#[derive(Debug, Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    password: Option<String>,
    endpoints: Option<Vec<Endpoint>>,
}

#[derive(Debug, Deserialize)]
struct Endpoint {
    path: Option<String>,
}

fn read_toml_config() {
    // read file
    let file_path =
        env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + "src/config.toml";
    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e),
    };

    // convert toml to string
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };

    // parse toml
    let config: Config = toml::from_str(&str_val).unwrap();
    println!(
        "ip: {}, port: {:#?}, password: {:#?}",
        config.ip, config.port, config.password
    );

    for x in config.endpoints.unwrap() {
        println!("{:#?}", x);
    }
}
