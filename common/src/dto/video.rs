use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoDto {
    pub name: String,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoListDto {
    pub video_list: Vec<VideoDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoSegmentDto {
    pub segment: Vec<u8>,
}
