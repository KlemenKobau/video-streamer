use common::{
    config::Config,
    dto::video::{Video, VideoSegment},
};
use tokio::fs;

use crate::errors::AppError;

pub async fn read_videos(config: &Config) -> Result<Vec<Video>, AppError> {
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
        .map(|(name, uuid)| Video {
            name: name.to_owned(),
            uuid: uuid.to_owned(),
        })
        .collect();

    Ok(videos)
}

pub async fn read_segment(
    config: &Config,
    video_id: String,
    segment_number: String,
) -> Result<VideoSegment, AppError> {
    let video_folder_path = config.video_config().video_folder_path();
    let file_path = format!("{}/{}/{}", video_folder_path, video_id, segment_number);
    let content = fs::read(file_path).await?;

    Ok(VideoSegment { segment: content })
}
