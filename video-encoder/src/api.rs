use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use common::config::Config;

use crate::{dto::video::VideoEncodeRequest, errors::AppError, video::encode_video};

#[post("encode")]
pub async fn submit_video(
    data: Data<Config>,
    video: Json<VideoEncodeRequest>,
) -> Result<impl Responder, AppError> {
    encode_video(&data, video.into_inner()).await;

    Ok(HttpResponse::Ok())
}
