use poem::web::Data;
use poem_openapi::{payload::Json, OpenApi};

use crate::{config::Config, dto::video_list::VideoList, video::read_videos};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/videos", method = "get")]
    async fn index(&self, conf_data: Data<&Config>) -> Result<Json<VideoList>> {
        let video_list = read_videos(conf_data.0).await?;

        Ok(Json(VideoList {
            video_list: video_list,
        }))
    }
}
