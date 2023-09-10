use std::io::Write;

use common::config::Config;
use tokio::{
    fs::{create_dir, File, OpenOptions},
    io::AsyncWriteExt,
    process::Command,
};
use uuid::Uuid;

use crate::errors::AppError;

pub async fn encode_video(config: &Config, video: &[u8]) -> Result<(), AppError> {
    let video_uuid = Uuid::new_v4();
    let video_path = format!(
        "{}/{}",
        config.encode_config().temp_video_folder(),
        video_uuid
    );

    save_video_to_temp(video, &video_path).await?;

    video_ffmpeg(config, &video_uuid, &video_path).await?;

    add_video_to_video_file(config, &video_uuid).await?;

    Ok(())
}

async fn save_video_to_temp(video: &[u8], video_file_path: &String) -> Result<File, AppError> {
    let mut file = File::create(video_file_path).await?;
    file.write_all(video).await?;

    Ok(file)
}

async fn video_ffmpeg(
    config: &Config,
    video_uuid: &Uuid,
    video_file_path: &String,
) -> Result<(), AppError> {
    let video_folder_path = format!(
        "{}/{}",
        config.video_config().video_folder_path(),
        video_uuid
    );

    create_dir(&video_folder_path).await?;

    Command::new("ffmpeg")
        .args(["-i", video_file_path])
        .args(["-codec", "copy"])
        .args(["-start_number", "0"])
        .args(["-hls_time", "7"])
        .args(["-hls_list_size", "0"])
        .args(["-f", "hls"])
        .args([
            "-hls_segment_filename",
            &format!("{}/filename%04d.hls", video_folder_path),
        ])
        .arg(&format!("{}/filename.m3u8", video_folder_path))
        .output()
        .await?;

    Ok(())
}

async fn add_video_to_video_file(config: &Config, video_uuid: &Uuid) -> Result<(), AppError> {
    let videos_folder = config.video_config().video_folder_path();
    let videos_file = config.video_config().videos_file();
    let file_path = format!("{}/{}", videos_folder, videos_file);

    let mut file = OpenOptions::new().append(true).open(file_path).await?;

    let mut vec: Vec<u8> = Vec::new();
    writeln!(vec, "name;{}", video_uuid)?;

    file.write_all(&vec).await?;

    Ok(())
}
