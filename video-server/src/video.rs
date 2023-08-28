use anyhow::Result;
use tokio::fs;

use crate::config::Config;

pub async fn read_videos(config: &Config) -> Result<Vec<String>> {
    let video_folder_path = config.video_config().video_folder_path();
    let video_file_name = config.video_config().videos_file();
    let file_path = format!("{}/{}", video_folder_path, video_file_name);

    let content = fs::read(file_path).await?;
    let parsed = String::from_utf8(content)?;
    let videos: Vec<String> = parsed.split("\n").map(|x| x.to_owned()).collect();

    Ok(videos)
}
