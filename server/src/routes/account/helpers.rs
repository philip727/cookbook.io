use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::HttpResponse;
use serde::Deserialize;

use crate::{
    database::models::user_details::UserDetails, helpers::is_alnum_whitespace_and_ex_chars, pretty_error, routes::error::PrettyErrorResponse
};

#[derive(Deserialize)]
pub struct UpdateUserDetailsPayload {
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub location: Option<String>,
}

impl UpdateUserDetailsPayload {
    pub fn verify(&self) -> Result<(), HttpResponse> {

        if let Some(pronouns) = &self.pronouns {
            if !UserDetails::is_pronoun(pronouns) {
                pretty_error!(
                    "This pronoun is invalid".to_string(),
                    "Please follow the rule of 'pronoun1/pronoun2'",
                    error
                );

                return Err(HttpResponse::BadRequest().json(error));
            }
        }

        if let Some(bio) = &self.bio {
            if !is_alnum_whitespace_and_ex_chars(bio) {
                pretty_error!(
                    "This bio is invalid".to_string(),
                    "Please only use alphanumerical characters",
                    error
                );
                return Err(HttpResponse::BadRequest().json(error));
            }
        }

        if let Some(location) = &self.location {
            if !is_alnum_whitespace_and_ex_chars(location) {
                pretty_error!(
                    "This location name is invalid".to_string(),
                    "Please only use alphanumerical characters",
                    error
                );

                return Err(HttpResponse::BadRequest().json(error));
            }
        }

        Ok(())
    }
}

#[derive(Debug, MultipartForm)]
pub struct UploadPictureForm {
    #[multipart(limit = "2MB")]
    pub picture: Option<TempFile>,
    pub details: Text<String>,
}
