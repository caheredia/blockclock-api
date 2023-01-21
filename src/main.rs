use std::thread::sleep;
use std::time::Duration;
mod time_utils;
use time_utils::{formatted_date, get_now};
mod toml_utils;
use toml_utils::{read_toml_file, read_urls_from_toml_config};

fn main() {
    let toml_str = read_toml_file();
    let urls = read_urls_from_toml_config(toml_str);
    println!("Calling endpoints:");

    for url in &urls {
        println!("\t {:#?}", url)
    }

    let timestamp = get_now();
    formatted_date(timestamp);

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
