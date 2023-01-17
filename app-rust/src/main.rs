use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize, Debug)]
struct Weather {
    name: String,
    main: Main,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: f32,
    humidity: f32,
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let api_key = "8d668dc35b99b78b988b0cad735ed49b";
    let cities = vec!["Brussels", "Antwerp", "Ghent", "Charleroi", "Liège", "Bruges", "Namur", "Leuven", "Mons", "Aalst"];

    for city in cities {
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={},BE&appid={}", city, api_key);
        let resp = client.get(&url)
            .send()
            .await
            .unwrap();
        let weather: Weather = from_str(&resp.text().await.unwrap()).unwrap();
        println!("{:?}", weather);
    }
}
