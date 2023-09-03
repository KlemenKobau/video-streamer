use std::{io, num::ParseIntError, string::FromUtf8Error};

use actix_web::{http::StatusCode, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InternalError {
    #[error("File system error.")]
    IOError(#[from] io::Error),

    #[error("File system error.")]
    ConfigError(#[from] FromUtf8Error),

    #[error("Cannot parse int.")]
    ParseIntError(#[from] ParseIntError),
}

impl ResponseError for InternalError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
