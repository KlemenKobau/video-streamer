use std::{io, num::ParseIntError};

use actix_web::{http::StatusCode, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error.")]
    IO(#[from] io::Error),

    #[error("Cannot parse int.")]
    ParseInt(#[from] ParseIntError),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
