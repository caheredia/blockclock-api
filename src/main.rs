use std::thread::sleep;
use std::time::Duration;
mod time_utils;
use time_utils::{current_time, formatted_date};
mod toml_utils;
use toml_utils::read_urls_from_toml_config;

fn main() {
    current_time(None);
    formatted_date(None);
    let urls = read_urls_from_toml_config(None);
    println!("Calling endpoints:");
    for url in &urls {
        println!("\t {:#?}", url)
    }
    loop {
        for url in &urls {
            println!("URL: {:#?}", url);
            make_request(url);
            sleep(Duration::from_secs(5 * 60));
        }
    }
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
