use actix_web::{error::ErrorUnauthorized, FromRequest, HttpMessage, HttpResponse};
use futures::future::{self, Ready};

use crate::middleware::auth::AuthenticationExtension;

pub enum Authorized {
    // Returns UID and Username
    Passed(i32, String),
    Failed(String),
}

impl FromRequest for Authorized {
    type Future = Ready<Result<Self, Self::Error>>;
    type Error = actix_web::Error;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let extensions = req.extensions();
        let auth = extensions.get::<AuthenticationExtension>();
        let Some(auth) = auth else {
            return future::ready(Ok(Authorized::Failed("Unable to authenticate JWT".into())));
        };

        let Ok(uid) = auth.uid.parse::<i32>() else {
            return future::ready(Ok(Authorized::Failed(
                "Unable to parse uid passed in bearer token".into(),
            )));
        };

        future::ready(Ok(Authorized::Passed(uid, auth.username.clone())))
    }
}
