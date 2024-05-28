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

#[macro_export]
macro_rules! pretty_error {
    ($title:expr, $description:expr, $err:ident) => {
        let $err = PrettyErrorResponse::new($title.into(), $description.into());
    };
}
