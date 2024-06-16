use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserDetailsPayload {
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, MultipartForm)]
pub struct UploadPictureForm {
    #[multipart(limit = "2MB")]
    pub picture: Option<TempFile>,
    pub details: Text<String>
}
