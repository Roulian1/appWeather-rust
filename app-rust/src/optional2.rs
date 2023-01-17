use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;
use std::io;

#[derive(Deserialize, Debug)]
struct Weather {
    name: String,
    main: Main,
    // add forecast for tomorrow and the day after tomorrow
    forecast: Forecast,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: f32,
    humidity: f32,
}

#[derive(Deserialize, Debug)]
struct Forecast {
    tomorrow: f32,
    after_tomorrow: f32,
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let api_key = "8d668dc35b99b78b988b0cad735ed49b";
    let cities = vec!["Brussels", "Antwerp", "Ghent", "Charleroi", "Liège", "Bruges", "Namur", "Leuven", "Mons", "Aalst"];
    let mut city = String::new();
    let mut forecast = String::new();

    println!("Select an option:");
    println!("1. Select one city from the list to get the weather forecast");
    println!("2. Get the weather forecast of our favourite city (unknown by the app)");
    println!("3. Add a command to get the weather forecast for tomorrow and the day after tomorrow");

    io::stdin().read_line(&mut city).unwrap();
    let city = city.trim();

    match city {
        "1" => {
            println!("Select a city:");
            for (index, city) in cities.iter().enumerate() {
                println!("{}. {}", index + 1, city);
            }
            io::stdin().read_line(&mut city).unwrap();
            let city = city.trim();
    
            println!("Do you want to get the weather forecast for tomorrow and the day after tomorrow? (y/n)");
            io::stdin().read_line(&mut forecast).unwrap();
            let forecast = forecast.trim();
    
            let url = match forecast {
                "y" => format!("https://api.openweathermap.org/data/2.5/forecast?q={},BE&appid={}", city, api_key),
                "n" => format!("https://api.openweathermap.org/data/2.5/weather?q={},BE&appid={}", city, api_key),
                _ => format!("https://api.openweathermap.org/data/2.5/weather?q={},BE&appid={}", city, api_key),
            };
    
            let resp = client.get(&url)
                .send()
                .await
                .unwrap();
            let weather: Weather = from_str(&resp.text().await.unwrap()).unwrap();
            println!("{:?}", weather);
        },
        "2" => {
            println!("Enter your favourite city:");
            io::stdin().read_line(&mut city).unwrap();
            let city = city.trim();
    
            println!("Do you want to get the weather forecast for tomorrow and the day after tomorrow? (y/n)");
            io::stdin().read_line(&mut forecast).unwrap();
            let forecast = forecast.trim();
    
            let url = match forecast {
                "y" => format!("https://api.openweathermap.org/data/2.5/forecast?q={},BE&appid={}", city, api_key),
                "n" => format!("https://api.openweathermap.org/data/2.5/weather?q={},BE&appid={}", city, api_key),
                _ => format!("https://api.openweathermap.org/data/2.5/weather?q={},BE&appid={}", city, api_key),
            };
    
            let resp = client.get(&url)
                .send()
                .await
                .unwrap();
            let weather: Weather = from_str(&resp.text().await.unwrap()).unwrap();
            println!("{:?}", weather);
        },
        _ => println!("Invalid option"),
    }
}