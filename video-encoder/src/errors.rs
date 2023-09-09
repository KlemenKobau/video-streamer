use std::{io, num::ParseIntError};

use actix_web::{http::StatusCode, ResponseError};
use base64::DecodeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error.")]
    IO(#[from] io::Error),

    #[error("Cannot parse int.")]
    ParseInt(#[from] ParseIntError),

    #[error("Decode error.")]
    Decode(#[from] DecodeError),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
