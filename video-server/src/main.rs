use actix_web::{
    middleware::Logger,
    web::{scope, Data},
    App, HttpServer,
};
use api::{video_file, video_segment, videos};
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
            .wrap(Logger::default())
            .app_data(config.clone())
            .service(
                scope("/api")
                    .service(videos)
                    .service(video_file)
                    .service(video_segment),
            )
    })
    .bind((host, port.parse::<u16>()?))?
    .run()
    .await?;

    Ok(())
}
