use std::{collections::BTreeMap, rc::Rc};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    web, HttpMessage,
};
use futures::future::{ok, ready, Either, LocalBoxFuture, Ready};
use sqlx::{Pool, Postgres};

use crate::{auth::helpers::verify_jwt_token, database::models::user::User};

pub struct AuthExtension {
    pub uid: String,
    pub username: String,
}

pub struct AuthMiddleware;

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
        })
    }
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let pool = req.app_data::<web::Data<Pool<Postgres>>>().cloned();
            let Some(pool) = pool else {
                return Err(ErrorInternalServerError(
                    "No database connection when trying to authorize bearer token",
                ));
            };

            let Some(auth_header) = req.headers().get("Authorization") else {
                return Err(ErrorUnauthorized("No bearer token passed"));
            };

            let Ok(auth_str) = auth_header.to_str() else {
                return Err(ErrorUnauthorized("Failed to parse bearer token"));
            };

            if !auth_str.starts_with("Bearer ") {
                return Err(ErrorUnauthorized("Invalid bearer token"));
            };

            let token = &auth_str[7..];
            let try_jwt = verify_jwt_token(token);
            if let Err(e) = try_jwt {
                return Err(ErrorUnauthorized(e.to_string()));
            };

            // Insert uid and username into the request extensions
            let (uid, username) = try_jwt.unwrap();
            let Ok(uid_int) = uid.parse::<i32>() else {
                return Err(ErrorUnauthorized(
                    "Failed to parse uid in bear authorization",
                ));
            };

            let user_exists = User::exists(&pool, uid_int).await;
            if let Err(e) = user_exists {
                return Err(ErrorInternalServerError(e.to_string()));
            };

            let user_exists = user_exists.unwrap();
            if !user_exists {
                return Err(ErrorInternalServerError(
                    "The uid claim provides an invalid user id",
                ));
            }

            let extension = AuthExtension { uid, username };
            req.extensions_mut().insert(extension);

            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}
