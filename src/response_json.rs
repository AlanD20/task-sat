use rocket::{form::FromForm, serde::json::Json};
use serde::Serialize;

#[derive(FromForm, Serialize, Debug)]
pub struct Response<Query, Data> {
    pub message: String,
    pub query: Query,
    pub data: Data,
}

impl<Query, Data> Response<Query, Data> {
    pub fn json(message: &str, query: Query, data: Data) -> Json<Response<Query, Data>> {
        Json(Response {
            message: message.to_string(),
            query,
            data,
        })
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: String,
}

impl ErrorResponse {
    pub fn new(message: &str, status: &str) -> Json<ErrorResponse> {
        Json(ErrorResponse {
            status: status.to_string(),
            message: message.to_string(),
        })
    }
}
