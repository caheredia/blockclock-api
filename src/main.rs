#[tokio::main]
async fn main() {
    let result = reqwest::get("http://192.168.10.196/api/show/text/HiBaby").await;
    println!("{:#?}", result)
}
