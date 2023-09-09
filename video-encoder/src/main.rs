use actix_web::{web::Data, App, HttpServer};
use api::submit_video;
use common::{config::Config, envconfig::Envconfig};
use errors::AppError;

mod api;
mod dto;
mod errors;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // ffmpeg -i video-world-war-2.mp4 -codec: copy -start_number 0 -hls_time 7 -hls_list_size 0 -f hls -hls_segment_filename output/filename%04d.hls filename.m3u8
    let config = Config::init_from_env().unwrap();

    let host: String = config.host().clone();
    let port = config.port().clone();
    let config = Data::new(config);

    HttpServer::new(move || App::new().app_data(config.clone()).service(submit_video))
        .bind((host, port.parse::<u16>()?))?
        .run()
        .await?;

    Ok(())
}
