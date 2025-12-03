use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum HypervisorError {
    #[error(transparent)]
    Any(#[from] anyhow::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl IntoResponse for HypervisorError {
    fn into_response(self) -> Response {
        let (status_code, err_msg) = match self {
            HypervisorError::Any(e) => {
                let status_code = e
                    .downcast_ref::<StatusCode>()
                    .map(ToOwned::to_owned)
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

                let msg = e.to_string();
                
                // Log the error with full context chain
                if status_code.is_server_error() {
                    tracing::error!("Server error ({}): {:?}", status_code, e);
                } else if status_code.is_client_error() {
                    tracing::warn!("Client error ({}): {}", status_code, msg);
                }

                (status_code, msg)
            }
            #[rustfmt::skip]
            HypervisorError::Io(e) => {
                tracing::error!("IO error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        };

        let err_resp = ErrorResponse { msg: err_msg };

        (status_code, axum::Json(err_resp)).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    msg: String,
}
