mod apires;

use std::process::exit;

use apires::{IpResponse, LocationResponse, WeatherResponse, WeatherResponseLive};
use reqwest::{blocking::Client, Error};

const API_KEY: &str = env!("AKEY");

fn ignore_err<T>(result: Result<T, Error>) -> T {
    match result {
        Ok(r) => r,
        Err(_) => {
            println!("{}", "I am the storm that is approaching");
            exit(1);
        }
    }
}

fn main() {
    let ip = ignore_err(get_ip());
    let location = ignore_err(get_location_by_ip(&ip));
    let weather = ignore_err(get_weather_by_location(&location));
    println!(
        "{} {} {}°C",
        weather.city, weather.weather, weather.temperature_float
    );
}

fn get_ip() -> Result<String, Error> {
    let client = Client::builder().no_proxy().build()?;
    let res = client.get("http://ip-api.com/json/").send()?;
    let ip: IpResponse = res.json()?;
    Ok(ip.query)
}

fn get_location_by_ip(ip: &str) -> Result<String, Error> {
    let client = Client::builder().no_proxy().build()?;
    let res = client
        .get(&format!(
            "https://restapi.amap.com/v3/ip?ip={}&output=json&key={}",
            ip, API_KEY
        ))
        .send()?;
    let location: LocationResponse = res.json()?;
    Ok(location.adcode)
}

fn get_weather_by_location(location: &str) -> Result<WeatherResponseLive, Error> {
    let client = Client::builder().no_proxy().build()?;
    let res = client
        .get(&format!(
            "https://restapi.amap.com/v3/weather/weatherInfo?city={}&key={}",
            location, API_KEY
        ))
        .send()?;
    let weather: WeatherResponse = res.json()?;
    Ok(weather.lives[0].clone())
}
