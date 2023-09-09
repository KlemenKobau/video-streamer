use actix_web::{web::Data, App, HttpServer};
use api::videos;
use common::{config::Config, envconfig::Envconfig};
use errors::AppError;

mod api;
mod dto;
mod errors;
mod video;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = Config::init_from_env().unwrap();

    let host: String = config.host().clone();
    let port = config.port().clone();
    let config = Data::new(config);

    HttpServer::new(move || App::new().app_data(config.clone()).service(videos))
        .bind((host, port.parse::<u16>()?))?
        .run()
        .await?;

    Ok(())
}
