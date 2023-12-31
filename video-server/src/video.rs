use common::{config::Config, dto::video::VideoDto};
use tokio::fs;
use tracing::info;

use crate::errors::AppError;

pub async fn read_videos(config: &Config) -> Result<Vec<VideoDto>, AppError> {
    let video_folder_path = config.video_config().video_folder_path();
    let video_file_name = config.video_config().videos_file();
    let file_path = format!("{}/{}", video_folder_path, video_file_name);

    let content = fs::read(file_path).await?;
    let parsed = String::from_utf8(content)?;
    let videos: Vec<_> = parsed
        .split('\n')
        .filter(|&x| !x.is_empty())
        .map(|x| {
            x.split_once(';')
                .expect("The videos should be in the correct format.")
        })
        .map(|(name, uuid)| VideoDto {
            name: name.to_owned(),
            uuid: uuid.to_owned(),
        })
        .collect();

    Ok(videos)
}

pub async fn read_file(config: &Config, video_id: String) -> Result<String, AppError> {
    let video_folder_path = config.video_config().video_folder_path();
    let file_path = format!("{}/{}/{}", video_folder_path, video_id, "filename.m3u8");
    let content = fs::read(file_path).await?;

    Ok(String::from_utf8(content)?)
}

pub async fn read_segment(
    config: &Config,
    video_id: String,
    segment_number: String,
) -> Result<Vec<u8>, AppError> {
    let video_folder_path = config.video_config().video_folder_path();
    let file_path = format!("{}/{}/{}", video_folder_path, video_id, segment_number);

    let content = fs::read(file_path).await?;

    Ok(content)
}
