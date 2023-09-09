use errors::AppError;

mod api;
mod errors;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // ffmpeg -i video-world-war-2.mp4 -codec: copy -start_number 0 -hls_time 7 -hls_list_size 0 -f hls -hls_segment_filename output/filename%04d.hls filename.m3u8

    Ok(())
}
