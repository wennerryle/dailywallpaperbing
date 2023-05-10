#![windows_subsystem = "windows"]

extern crate serde;
extern crate serde_json;
extern crate reqwest;
use wallpaper;


#[derive(Debug, serde::Deserialize)]
struct BingResult {
    url: String,
}

fn main() {
    let api_url = "https://bing.biturl.top";
    let client = reqwest::blocking::Client::new();

    // выполняем запрос к API и десериализуем JSON-ответ
    let response = client.get(&format!("{}", api_url)).send().unwrap();
    let url_image: BingResult = serde_json::from_str(&response.text().unwrap()).unwrap();

    wallpaper::set_from_url(&url_image.url).unwrap();
}