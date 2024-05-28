use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PrettyErrorResponse {
    error: String,
    description: String,
}

impl PrettyErrorResponse {
    pub fn new(title: String, description: String) -> Self {
        PrettyErrorResponse {
            error: title,
            description
        }
    }
}
