use super::content_type::JSON;
use axum::{
    Json,
    http::header,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub(super) struct JsonResponse<T>(pub(in crate::http) T);

impl<T> IntoResponse for JsonResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        ([(header::CONTENT_TYPE, JSON)], Json(self.0)).into_response()
    }
}
