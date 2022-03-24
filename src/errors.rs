use std::fmt;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::error::BlockingError;
use diesel::r2d2;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error;
use diesel::result::Error::DatabaseError;
use serde::{Serialize};

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
    NotUniqueValue
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}
impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                ..
            } => message.clone(),
            AppError {
                message: None,
                error_type: AppErrorType::NotUniqueValue,
                ..
            } => "Указанное значение не уникально".to_string(),
            AppError {
                message: None,
                error_type: AppErrorType::NotFoundError,
                ..
            } => "The requested item was not found".to_string(),
            _ => "Произошла ошибка".to_string(),
        }
    }
}

impl From<r2d2::PoolError> for AppError {
    fn from(error: r2d2::PoolError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError
        }
    }
}

impl From<Error> for AppError {
    fn from(error: Error) -> AppError {
        if let DatabaseError(kind, _) = error {
            if let UniqueViolation = kind {
                return AppError {
                    message: None,
                    cause: Some(error.to_string()),
                    error_type: AppErrorType::NotUniqueValue
                }
            }
        }
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError
        }
    }
}
impl From<BlockingError<Error>> for AppError {
    fn from(error: BlockingError<Error>) -> AppError {
        match error {
            BlockingError::Error(e) => AppError::from(e),
            BlockingError::Canceled => AppError {
                message: None,
                cause: Some("Ошибка пула потоков".to_string()),
                error_type: AppErrorType::DbError
            }
        }
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::NotUniqueValue => StatusCode::BAD_REQUEST
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}
