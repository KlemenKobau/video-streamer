use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Video {
    pub name: String,
    pub uuid: String,
}

#[derive(Debug, Serialize)]
pub struct VideoList {
    pub video_list: Vec<Video>,
}

#[derive(Debug, Serialize)]
pub struct VideoSegment {
    pub segment: Vec<u8>,
}
