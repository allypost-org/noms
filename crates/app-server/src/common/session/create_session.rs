use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::http::{Request, Response};
use tower::Service;

// middleware that
// 1. inserts a session into request extensions so handlers can access it
// 2. calls the inner service and awaits the response
// 3. prints the session so we can see if its changed
#[derive(Clone)]
pub struct CreateSessionHandler<T, TS, S>
where
    T: Fn() -> TS + Send + 'static + Clone,
{
    inner: S,
    make_session: T,
}

impl<T, TS, S> CreateSessionHandler<T, TS, S>
where
    T: Fn() -> TS + Send + 'static + Clone,
{
    pub fn new(inner: S, make_session: T) -> Self {
        Self {
            inner,
            make_session,
        }
    }
}

impl<T, TS, S, ReqBody, ResBody> Service<Request<ReqBody>> for CreateSessionHandler<T, TS, S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
    T: Fn() -> TS + Send + 'static + std::marker::Sync + Clone,
    TS: Clone + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;

    #[allow(clippy::type_complexity)]
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        // best practice is to clone the inner service like this
        // see https://github.com/tower-rs/tower/issues/547 for details
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let session = (self.make_session)();

        Box::pin(async move {
            req.extensions_mut().insert(session);

            inner.call(req).await
        })
    }
}
