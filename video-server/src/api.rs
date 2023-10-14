use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
};
use common::{config::Config, dto::video::VideoListDto};
use tracing::info;

use crate::{
    errors::AppError,
    video::{read_file, read_segment, read_videos},
};

#[get("/videos")]
pub async fn videos(data: Data<Config>) -> Result<impl Responder, AppError> {
    let videos = read_videos(&data).await?;
    let videos = VideoListDto { video_list: videos };

    Ok(Json(videos))
}

#[get("/videos/{video_id}/filename.m3u8")]
pub async fn video_file(
    data: Data<Config>,
    path: Path<String>,
) -> Result<impl Responder, AppError> {
    info!("Request for video file");

    let video_id = path.into_inner();
    let video_file = read_file(&data, video_id).await?;

    Ok(video_file)
}

#[get("/videos/{video_id}/{segment_number}")]
pub async fn video_segment(
    data: Data<Config>,
    path: Path<(String, String)>,
) -> Result<impl Responder, AppError> {
    info!("Request for segment");

    let (video_id, segment_number) = path.into_inner();
    let video_segment = read_segment(&data, video_id, segment_number).await?;

    Ok(video_segment)
}
