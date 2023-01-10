//! this module inserts cache headers on outgoing responses
use std::future::{ready, Ready};
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::http::StatusCode;
use futures_util::future::LocalBoxFuture;

///This struct exists only for the InsertCacheHeaderMiddleware.
pub struct InsertCacheHeader;

impl<S, B> Transform<S, ServiceRequest> for InsertCacheHeader
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = InsertCacheHeaderMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InsertCacheHeaderMiddleware { service }))
    }
}

/// `InsertCacheHeaderMiddleware` is a middleware that inserts cache headers into the response.
pub struct InsertCacheHeaderMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for InsertCacheHeaderMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            if res.status().eq(&StatusCode::OK) || res.status().eq(&StatusCode::CREATED) {
                res.headers_mut().insert(HeaderName::from_static("cache-control"), HeaderValue::from_static("no-cache"));
            }
            Ok(res)
        })
    }
}