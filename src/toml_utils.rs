use serde_derive::Deserialize;
use std::env;
use std::fs::File;
use std::io::prelude::*;

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

fn read_toml_file() -> String {
    // read file
    let file_path =
        env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + "src/config.toml";
    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e),
    };

    // convert toml to string
    let mut toml_str = String::new();
    match file.read_to_string(&mut toml_str) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };
    return toml_str;
}

pub fn read_urls_from_toml_config() -> Vec<String> {
    let toml_str = read_toml_file();
    // parse toml
    let config: Config = match toml::from_str(&toml_str) {
        Ok(config) => config,
        Err(_) => panic!("Invalid config, can't parse to string"),
    };
    println!(
        "ip: {}, port: {:#?}, password: {:#?}",
        config.ip, config.port, config.password
    );

    let endpoints = match config.endpoints {
        Some(e) => e,
        None => panic!("No Endpoints found"),
    };

    // append ip to path
    let mut urls = Vec::new();
    for endpoint in endpoints {
        match endpoint.path {
            Some(path) => urls.push(config.ip.to_owned() + &path),
            None => continue,
        }
    }
    return urls;
}
