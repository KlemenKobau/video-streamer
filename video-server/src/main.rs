use actix_web::{App, HttpServer};
use api::videos;
use envconfig::Envconfig;
use errors::InternalError;

mod api;
mod config;
mod dto;
mod errors;
mod video;

#[tokio::main]
async fn main() -> Result<(), InternalError> {
    let config = config::Config::init_from_env().unwrap();

    HttpServer::new(|| App::new().service(videos))
        .bind((config.host().to_owned(), config.port().parse()?))?
        .run()
        .await?;

    Ok(())
}
