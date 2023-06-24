use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use tokio::task::JoinError;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    ParseUuid(String),

    #[error("{0}")]
    Authenticate(#[from] AuthenticateError),

    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    NotFound(#[from] NotFound),

    #[error("{0}")]
    RunSyncTask(#[from] JoinError),

    #[error("{0}")]
    Repo(#[from] RepoError),

    #[error("{0}")]
    Database(#[from] sqlx::Error),

    #[error("{0}")]
    ZeroKnowledgeDatabase(#[from] liserk_client::Error),

    #[error("{0}")]
    Fetch(#[from] reqwest::Error),

    #[error("{0}")]
    InteractionError(#[from] InteractionError),
}

impl Error {
    fn get_codes(&self) -> (StatusCode, u16) {
        match *self {
            // 4XX Errors
            Error::ParseUuid(_) => (StatusCode::BAD_REQUEST, 40001),
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, 40002),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 40003),
            Error::Authenticate(AuthenticateError::WrongCredentials) => {
                (StatusCode::UNAUTHORIZED, 40004)
            }
            Error::Authenticate(AuthenticateError::InvalidToken) => {
                (StatusCode::UNAUTHORIZED, 40005)
            }
            Error::Authenticate(AuthenticateError::Locked) => (StatusCode::LOCKED, 40006),
            Error::InteractionError(_) => (StatusCode::BAD_REQUEST, 40007),

            // 5XX Errors
            Error::Authenticate(AuthenticateError::TokenCreation) => {
                (StatusCode::INTERNAL_SERVER_ERROR, 5001)
            }
            Error::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5005),
            Error::Repo(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5006),
            Error::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5007),
            Error::Fetch(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5008),
            Error::ZeroKnowledgeDatabase(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5009),
        }
    }

    pub fn bad_request() -> Self {
        Error::BadRequest(BadRequest {})
    }

    pub fn not_found() -> Self {
        Error::NotFound(NotFound {})
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, code) = self.get_codes();
        let message = self.to_string();
        let json = match &self {
            Error::InteractionError(error) => {
                json!({ "code": code, "message": message, "interactions": error.interactions })
            }
            _ => json!({ "code": code, "message": message }),
        };
        let body = Json(json);

        (status_code, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AuthenticateError {
    #[error("Wrong authentication credentials")]
    WrongCredentials,
    #[error("Failed to create authentication token")]
    TokenCreation,
    #[error("Invalid authentication credentials")]
    InvalidToken,
    #[error("User is locked")]
    Locked,
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request")]
pub struct BadRequest {}

#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFound {}

#[derive(thiserror::Error, Debug)]
#[error("Repository error")]
pub struct RepoError {}

#[derive(thiserror::Error, Debug)]
#[error("Interaction between medication found")]
pub struct InteractionError {
    pub message: String,
    pub interactions: Vec<Vec<String>>,
}
