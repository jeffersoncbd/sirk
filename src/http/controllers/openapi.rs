use super::super::{openapi::OPENAPI, types::TEXT_CONTENT_TYPE};
use axum::{
    http::header,
    response::{IntoResponse, Response},
};

pub(in crate::http) async fn openapi() -> Response {
    ([(header::CONTENT_TYPE, TEXT_CONTENT_TYPE)], OPENAPI).into_response()
}
