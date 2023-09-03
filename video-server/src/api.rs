use actix_web::{
    get,
    web::{Data, Json},
    Responder,
};

use crate::{config::Config, errors::InternalError, video::read_videos};

#[get("/videos")]
pub async fn videos(data: Data<Config>) -> Result<impl Responder, InternalError> {
    let videos = read_videos(&data).await?;

    Ok(Json(videos))
}
