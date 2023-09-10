use std::{io, num::ParseIntError};

use actix_web::{error::PayloadError, http::StatusCode, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error.")]
    IO(#[from] io::Error),

    #[error("Cannot parse int.")]
    ParseInt(#[from] ParseIntError),

    #[error("Payload error.")]
    Payload(#[from] PayloadError),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::IO(_) | AppError::ParseInt(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Payload(_) => StatusCode::BAD_REQUEST,
        }
    }
}
