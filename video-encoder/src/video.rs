use base64::{engine::general_purpose, Engine};
use common::config::Config;
use tokio::{
    fs::{create_dir, File},
    io::AsyncWriteExt,
    process::Command,
};
use uuid::Uuid;

use crate::{dto::video::VideoEncodeRequest, errors::AppError};

pub async fn encode_video(
    config: &Config,
    video_encode_req: VideoEncodeRequest,
) -> Result<(), AppError> {
    let video_name = &video_encode_req.video_name;
    let video_uuid = Uuid::new_v4();
    let video_path = format!(
        "{}/{}",
        config.encode_config().temp_video_folder(),
        video_uuid
    );

    let temp_video_file = save_video_to_temp(&video_encode_req, &video_path).await?;

    Ok(())
}

async fn save_video_to_temp(
    video_encode_req: &VideoEncodeRequest,
    video_file_path: &String,
) -> Result<File, AppError> {
    let decoded_video = general_purpose::STANDARD.decode(&video_encode_req.video)?;

    let mut file = File::create(video_file_path).await?;
    file.write_all(&decoded_video).await?;

    Ok(file)
}

async fn video_ffmpeg(
    config: &Config,
    video_name: &String,
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
