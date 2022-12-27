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
/// Converts a toml file to a string
pub fn read_toml_file() -> String {
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

pub fn read_urls_from_toml_config(toml_str: String) -> Vec<String> {
    // parse toml
    let config: Config = match toml::from_str(&toml_str) {
        Ok(config) => config,
        Err(_) => panic!("Invalid config, can't parse to string"),
    };
    // println!(
    //     "ip: {}, port: {:#?}, password: {:#?}",
    //     config.ip, config.port, config.password
    // );

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
    urls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_urls_from_toml_config_positive_case() {
        let config_str = "
            ip='localhost/api/'
            [[endpoints]]
            path = 'show/text/Wow'
            [[endpoints]]
            path = 'pick/cm.markets.sats_per_dollar'
            ";
        assert_eq!(
            vec![
                "localhost/api/show/text/Wow",
                "localhost/api/pick/cm.markets.sats_per_dollar"
            ],
            read_urls_from_toml_config(config_str.to_string())
        );
    }

    #[test]
    #[should_panic(expected = "Invalid config, can't parse to string")]
    fn test_read_urls_from_toml_config_bad_config() {
        // bad config
        read_urls_from_toml_config("Not a valid config".to_string());
    }
    #[test]
    #[should_panic(expected = "No Endpoints found")]
    fn test_read_urls_from_toml_config_no_endpoints() {
        // missing endpoints
        read_urls_from_toml_config("ip='localhost/api/'".to_string());
    }
}
