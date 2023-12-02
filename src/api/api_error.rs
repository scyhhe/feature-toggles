use serde::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    code: u16,
    message: String,
}

impl ApiError {
    pub fn new(code: u16, message: String) -> Self {
        ApiError { code, message }
    }
}
