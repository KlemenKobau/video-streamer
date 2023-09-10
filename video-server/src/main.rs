use actix_web::{web::Data, App, HttpServer};
use api::{video_segment, videos};
use common::{config::Config, envconfig::Envconfig};
use errors::AppError;

mod api;
mod dto;
mod errors;
mod video;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();

    let config = Config::init_from_env().unwrap();

    let host: String = config.host().clone();
    let port = config.port().clone();
    let config = Data::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .service(videos)
            .service(video_segment)
    })
    .bind((host, port.parse::<u16>()?))?
    .run()
    .await?;

    Ok(())
}
