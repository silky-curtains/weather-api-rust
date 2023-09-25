use serde::Deserialize;
use serde::Serialize;

#[derive(Clone)]
pub struct WeatherApiConfig {
  // Base URL of openweathermap api
  pub base_url: String,
  // API Key
  pub app_id: String,
}

impl WeatherApiConfig {
  pub fn new (base_url: String, app_id: String) -> Self {
    WeatherApiConfig { base_url, app_id }
  }
}

#[derive(Clone)]
pub struct AppState {
  pub config: WeatherApiConfig
}

impl AppState {
  pub fn new (config: WeatherApiConfig) -> Self {
    AppState { config }
  }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Wind {
    speed: f64,
    deg: i32,
    gust: Option<f64>,
}
// #[derive(Debug, Deserialize, Serialize)]
// struct Rain {
//     #[serde(rename = "1h")]
//     data: f64,
// }

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Clouds {
    all: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Rain {
    #[serde(rename = "1h")]
    one_hour: f64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Sys {
    #[serde(rename = "type")]
    sys_type: i32,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherResponse {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    rain: Option<Rain>,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}
