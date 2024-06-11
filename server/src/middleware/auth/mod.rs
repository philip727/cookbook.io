use std::{marker::PhantomData, rc::Rc, task::Poll};

use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    HttpMessage, HttpResponse,
};
use futures::{
    future::{ok, Either, Ready},
    ready, Future,
};
use pin_project::pin_project;

use crate::auth::helpers::verify_jwt_token;

pub struct AuthenticationExtension {
    pub uid: String,
    pub username: String,
}

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = actix_web::Error;
    type Future = Either<AuthenticationFuture<S, B>, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let Some(auth_header) = req.headers().get("Authorization") else {
            let res = HttpResponse::with_body(StatusCode::UNAUTHORIZED, "No bearer token passed");
            return Either::Right(ok(req
                .into_response(res)
                .map_into_boxed_body()
                .map_into_right_body()));
        };

        let Ok(auth_str) = auth_header.to_str() else {
            let res = HttpResponse::with_body(
                StatusCode::UNAUTHORIZED,
                "Failed to parse bearer token to string",
            );
            return Either::Right(ok(req
                .into_response(res)
                .map_into_boxed_body()
                .map_into_right_body()));
        };

        if !auth_str.starts_with("Bearer ") {
            let res = HttpResponse::with_body(StatusCode::UNAUTHORIZED, "Invalid bearer token");
            return Either::Right(ok(req
                .into_response(res)
                .map_into_boxed_body()
                .map_into_right_body()));
        };

        let token = &auth_str[7..];
        let try_jwt = verify_jwt_token(token);
        if let Err(e) = try_jwt {
            let res = HttpResponse::with_body(StatusCode::UNAUTHORIZED, e.to_string());
            return Either::Right(ok(req
                .into_response(res)
                .map_into_boxed_body()
                .map_into_right_body()));
        };

        // Insert uid and username into the request extensions
        let (uid, username) = try_jwt.unwrap();
        let Ok(uid_int) = uid.parse::<i32>() else {
            let res = HttpResponse::with_body(
                StatusCode::UNAUTHORIZED,
                "Failed to parse uid passed in bearer token",
            );
            return Either::Right(ok(req
                .into_response(res)
                .map_into_boxed_body()
                .map_into_right_body()));
        };

        let extension = AuthenticationExtension { uid, username };
        req.extensions_mut().insert(extension);

        Either::Left(AuthenticationFuture {
            fut: self.service.call(req),
            _phantom: PhantomData,
        })
    }
}

#[pin_project]
pub struct AuthenticationFuture<S, B>
where
    S: Service<ServiceRequest>,
{
    #[pin]
    fut: S::Future,
    _phantom: PhantomData<B>,
}

impl<S, B> Future for AuthenticationFuture<S, B>
where
    B: MessageBody,
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Output = Result<ServiceResponse<EitherBody<B>>, actix_web::Error>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let res = match ready!(self.project().fut.poll(cx)) {
            Ok(res) => res,
            Err(err) => return Poll::Ready(Err(err.into())),
        };

        Poll::Ready(Ok(res.map_into_left_body()))
    }
}
