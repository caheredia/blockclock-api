use chrono::prelude::*;
use serde_derive::Deserialize;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let time = Local::now()
        .naive_local()
        .format("%a %b %e %T %Y %I %p")
        .to_string();
    println!("{:#?}", time);
    let urls = read_urls_from_toml_config();
    println!("Calling endpoints:");
    for url in &urls {
        println!("\t {:#?}", url)
    }
    loop {
        for url in &urls {
            println!("URL: {:#?}", url);
            // make_request(url);
            sleep(Duration::from_secs(5 * 60));
        }
    }
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

fn read_urls_from_toml_config() -> Vec<String> {
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
    let config: Config = match toml::from_str(&str_val) {
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

fn make_request(url: &str) {
    match ureq::get(url).call() {
        Ok(response) => println!("response: {:#?}", response),
        Err(ureq::Error::Status(code, response)) => {
            println!("code: {} response: {:#?}", code, response)
        }
        Err(_) => println!("Could not process request"),
    }
}
