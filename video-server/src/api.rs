use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
};

use crate::{config::Config, errors::AppError, video::{read_videos, read_segment}, dto::video::VideoList};

#[get("/videos")]
pub async fn videos(data: Data<Config>) -> Result<impl Responder, AppError> {
    let videos = read_videos(&data).await?;
    let videos = VideoList {video_list: videos};

    Ok(Json(videos))
}

#[get("/video/{video_id}/{segment_number}")]
pub async fn video_segment(data: Data<Config>, path: Path<(String, String)>) -> Result<impl Responder, AppError> {
    let (video_id, segment_number) = path.into_inner();
    let video_segment = read_segment(&data, video_id, segment_number).await?;

    Ok(Json(video_segment))
}
