use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct VideoList {
    pub video_list: Vec<String>,
}
