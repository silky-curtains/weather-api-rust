use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
mod models;
use models::{AppState, WeatherApiConfig, WeatherResponse};
use serde::{Deserialize, Serialize};
extern crate dotenv;
use dotenv::dotenv;

#[derive(Debug, Serialize, Deserialize)]
struct WeatherQuery {
    city: Option<String>,
    lat: Option<f64>,
    lon: Option<f64>,
}

async fn index(query: web::Query<WeatherQuery>, data: web::Data<AppState>) -> impl Responder {
    let WeatherQuery { city, lat, lon } = query.into_inner();
    let WeatherApiConfig { app_id, base_url } = &data.config;

    let url;

    match (&city, (&lat, &lon)) {
        (None, (Some(lat), Some(lon))) => {
            url = format!("{}?lat={}&lon={}&appid={}", base_url, lat, lon, app_id);
        }

        (Some(city), (None, None)) => {
            url = format!("{}?q={}&appid={}", base_url, city, app_id);
        }

        _ => {
            return HttpResponse::UnprocessableEntity().body(
                "The query parameters must either be both lattitude and longitude of a place, or a location name (like London). Not both."
            );
        }
    }

    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(_err) => {
            return HttpResponse::InternalServerError()
                .body("There was an error connecting to the Weather API");
        }
    };

    let status = response.status();
    
    if !status.is_success() {
        return HttpResponse::UnprocessableEntity()
            .body("Invalid request. Please ensure the city exists or the lattitude and longitude values are valid.");
    }

    let weather_data = match response.json::<WeatherResponse>().await {
        Ok(weather_data) => weather_data,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!(
                "Error deserializing json response from weather api. {}",
                err
            ));
        }
    };

    /*
     * I know what I'm doing here is basically deserializing a json to 
     * struct and then serializing it back to json, which is not really 
     * efficient
     */

    let body = match serde_json::to_string(&weather_data) {
        Ok(body) => body,
        Err(_err) => {
            return HttpResponse::InternalServerError()
                .body("Error serializing weather response to json")
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState::new(WeatherApiConfig::new(
                String::from("https://api.openweathermap.org/data/2.5/weather"),
                dotenv::var("APPID").unwrap(),
            ))))
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
