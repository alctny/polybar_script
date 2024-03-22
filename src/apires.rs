use serde::Deserialize;

#[derive(Deserialize)]
pub struct IpResponse {
    pub query: String,
}

#[derive(Deserialize)]
pub struct LocationResponse {
    pub adcode: String,
}

#[derive(Deserialize)]
pub struct WeatherResponse {
    pub lives: Vec<WeatherResponseLive>,
}

#[derive(Deserialize, Clone)]
pub struct WeatherResponseLive {
    pub temperature_float: String,
    pub weather: String,
    pub city: String,
}
