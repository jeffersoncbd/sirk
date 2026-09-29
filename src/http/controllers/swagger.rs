use super::super::{swagger::SWAGGER_UI, types::HTML_CONTENT_TYPE};
use axum::{
    http::header,
    response::{IntoResponse, Response},
};

pub(in crate::http) async fn swagger() -> Response {
    ([(header::CONTENT_TYPE, HTML_CONTENT_TYPE)], SWAGGER_UI).into_response()
}
