use candid::{CandidType, Deserialize};
use std::fmt::Display;

#[derive(Debug, CandidType, Deserialize)]
pub enum ApiResult<T = ()> {
    Ok(T),
    Err(ApiError),
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq)]
pub struct ApiError {
    code: u16,
    message: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

#[allow(dead_code)]
impl ApiError {
    pub fn invalid_argument(message: &str) -> Self {
        Self {
            code: 400,
            message: message.into(),
        }
    }

    pub fn unauthenticated() -> Self {
        Self {
            code: 401,
            message: "Anonymous principals are not allowed to call this endpoint".to_string(),
        }
    }

    pub fn permission_denied(message: &str) -> Self {
        Self {
            code: 403,
            message: message.into(),
        }
    }

    pub fn not_found(message: &str) -> Self {
        Self {
            code: 404,
            message: message.into(),
        }
    }

    pub fn conflict(message: &str) -> Self {
        Self {
            code: 409,
            message: message.into(),
        }
    }

    pub fn internal(message: &str) -> Self {
        Self {
            code: 500,
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl<T> From<Result<T, ApiError>> for ApiResult<T> {
    fn from(result: Result<T, ApiError>) -> Self {
        match result {
            Ok(value) => ApiResult::Ok(value),
            Err(err) => ApiResult::Err(err),
        }
    }
}
