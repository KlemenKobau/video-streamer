use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VideoEncodeRequest {
    pub video_name: String,
    pub video: String,
}
