use std::pin::Pin;
use std::task::{Context, Poll};

// use actix_service::{Service, Transform};
// use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
// use futures::future::{ok, Ready};
use futures::future::{ready, Ready};
use futures::Future;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SayHi;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S: Service<Req>, Req> Transform<S, Req> for SayHi
{
    type Response = S::Response;
    type Error = S::Error;
    type InitError = S::Error;
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S: Service<Req>, Req> Service<Req> for SayHiMiddleware<S>
where

{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }
    // actix_web::dev::forward_ready!(service);

    fn call(&self, req: Req) -> Self::Future {
        // println!("Hi from start. You requested: {}", req.path());
        println!("Hi from start. You requested: ");

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}
