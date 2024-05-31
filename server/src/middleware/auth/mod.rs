use std::{collections::BTreeMap, rc::Rc};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized, HttpMessage,
};
use futures::future::{ok, ready, Either, Ready};
use jwt::VerifyWithKey;

use crate::auth::helpers::verify_jwt_token;

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
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let Some(auth_header) = req.headers().get("Authorization") else {
            return Either::Right(ready(Err(ErrorUnauthorized("No bearer token passed"))));
        };

        let Ok(auth_str) = auth_header.to_str() else {
            return Either::Right(ready(Err(ErrorUnauthorized(
                "Failed to parse bearer token",
            ))));
        };

        if !auth_str.starts_with("Bearer ") {
            return Either::Right(ready(Err(ErrorUnauthorized("Invalid bearer token"))));
        };

        let token = &auth_str[7..];
        let try_jwt = verify_jwt_token(token);
        if let Err(e) = try_jwt {
            return Either::Right(ready(Err(ErrorUnauthorized(e.to_string()))));
        };

        // Insert uid and username into the request extensions
        let (uid, username) = try_jwt.unwrap();
        req.extensions_mut().insert(uid);
        req.extensions_mut().insert(username);

        Either::Left(self.service.call(req))
    }
}
