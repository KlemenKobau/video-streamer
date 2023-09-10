use actix_web::{
    post,
    web::{BytesMut, Data, Payload},
    HttpResponse, Responder,
};
use common::config::Config;
use futures::StreamExt;
use tracing::info;

use crate::{errors::AppError, video::encode_video};

#[post("encode")]
pub async fn submit_video(
    data: Data<Config>,
    mut video: Payload,
) -> Result<impl Responder, AppError> {
    info!("Got request");

    let mut bytes = BytesMut::new();

    while let Some(item) = video.next().await {
        let item = item?;
        bytes.extend_from_slice(&item);
    }

    let bytes = bytes.freeze();

    encode_video(&data, &bytes[..]).await?;

    Ok(HttpResponse::Ok())
}
