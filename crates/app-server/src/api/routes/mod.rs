use std::{sync::Arc, time::Duration};

use axum::{
    Extension, Router,
    extract::State,
    http::{HeaderMap, HeaderValue, Request, Response, header},
    routing::{any, get},
};
use axum_client_ip::ClientIpSource;
use jsonwebtoken::{DecodingKey, EncodingKey};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::{self, CorsLayer},
    request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    set_header::SetResponseHeaderLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::{Span, debug, field, info};

use crate::{common::session::CreateSessionHandler, config::ApiServerConfig};

#[derive(Clone)]
pub struct ApiState {
    jwt_encoding_key: EncodingKey,
    jwt_decoding_key: DecodingKey,
}

// example session type
#[derive(Debug, Default)]
pub struct Session {
    id: Arc<RwLock<String>>,
}

pub type SessionExt = Extension<Arc<Session>>;

pub fn create_router(config: &ApiServerConfig) -> Router {
    let state = Arc::new(ApiState {
        jwt_encoding_key: EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        jwt_decoding_key: DecodingKey::from_secret(config.jwt_secret.as_bytes()),
    });

    add_middlewares(
        Router::new()
            .route("/", any(handle_any_root))
            .route("/healthz", get(handle_healthz))
            .route("/plus-plus", any(handle_plus_plus))
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                create_session,
            ))
            .layer(tower::layer::layer_fn(|x| {
                CreateSessionHandler::new(x, || Arc::new(Session::default()))
            }))
            .with_state(state),
    )
}

async fn create_session(
    headers: HeaderMap,
    State(state): State<Arc<ApiState>>,
    Extension(session): SessionExt,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let token = headers
        .get("Authorization")
        .and_then(|x| x.to_str().ok())
        .and_then(|x| {
            let mut parts = x.split_whitespace();

            if let (Some(t), Some(token)) = (parts.next(), parts.next())
                && t.eq_ignore_ascii_case("Bearer")
                && !token.is_empty()
            {
                return Some((*token).to_string());
            }

            None
        })
        .or_else(|| {
            headers.get("X-Auth-Token").and_then(|header| {
                let str = header.to_str().unwrap_or_default();
                if str.is_empty() {
                    None
                } else {
                    Some(str.to_string())
                }
            })
        });

    let token = token.and_then(|token| {
        jsonwebtoken::decode::<Claims>(
            &token,
            &state.jwt_decoding_key,
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512),
        )
        .ok()
    });

    if let Some(token) = token {
        let claims = token.claims;

        *session.id.write().await = claims.sub;
    }

    next.run(request).await
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}

async fn handle_plus_plus(
    Extension(session): Extension<Arc<Session>>,
    State(state): State<Arc<ApiState>>,
) -> Result<String, axum::http::StatusCode> {
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512),
        &Claims {
            sub: rand::random::<u128>().to_string(),
            exp: jiff::Timestamp::now().as_second().saturating_add(300),
        },
        &state.jwt_encoding_key,
    )
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(format!("OK: {:?} | {:?}", session, token))
}

#[derive(Clone)]
struct MakeRequestUlid;
impl MakeRequestId for MakeRequestUlid {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let mut id = ulid::Ulid::generate().to_string();
        id.make_ascii_lowercase();
        let val = HeaderValue::from_str(&id).ok()?;

        Some(RequestId::new(val))
    }
}

fn add_middlewares<T>(router: Router<T>) -> Router<T>
where
    T: std::clone::Clone + Send + Sync + 'static,
{
    router
        .layer(CatchPanicLayer::new())
        .layer(
            ServiceBuilder::new()
                .layer(ClientIpSource::RightmostXForwardedFor.into_extension())
                .layer(SetRequestIdLayer::x_request_id(MakeRequestUlid))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            let m = request.method();
                            let p = request.uri().path();
                            let id = request
                                .extensions()
                                .get::<RequestId>()
                                .and_then(|id| id.header_value().to_str().ok())
                                .unwrap_or("-");
                            let dur = field::Empty;
                            let user = field::Empty;

                            tracing::info_span!("api", %id, %m, ?p, dur, user)
                        })
                        .on_request(|request: &Request<_>, _span: &Span| {
                            let headers = request.headers();
                            info!(
                                target: "request",
                                "START \"{method} {uri} {http_type:?}\" {user_agent:?} {ip:?}",
                                http_type = request.version(),
                                method = request.method(),
                                uri = request.uri().path(),
                                user_agent = headers
                                    .get(header::USER_AGENT)
                                    .map_or("-", |x| x.to_str().unwrap_or("-")),
                                ip = headers
                                    .get("x-forwarded-for")
                                    .map_or("-", |x| x.to_str().unwrap_or("-")),
                            );
                        })
                        .on_response(|response: &Response<_>, latency, span: &Span| {
                            span.record("dur", field::debug(latency));
                            info!(
                                target: "request",
                                "END {status}",
                                status = response.status().as_u16(),
                            );
                        })
                        .on_body_chunk(())
                        .on_eos(|_trailers: Option<&_>, stream_duration, span: &Span| {
                            span.record("dur", field::debug(stream_duration));
                            debug!(
                                target: "request",
                                "ERR: stream closed unexpectedly",
                            );
                        })
                        .on_failure(|error, latency, span: &Span| {
                            span.record("dur", field::debug(latency));
                            debug!(
                                target: "request",
                                err = ?error,
                                "ERR: something went wrong",
                            );
                        }),
                )
                .layer(PropagateRequestIdLayer::x_request_id())
                .layer(SetResponseHeaderLayer::appending(
                    header::DATE,
                    |_response: &Response<_>| {
                        jiff::fmt::rfc2822::DateTimePrinter::new()
                            .timestamp_to_rfc9110_string(&jiff::Timestamp::now())
                            .ok()
                            .and_then(|value| value.parse().ok())
                    },
                ))
                .layer(TimeoutLayer::with_status_code(
                    axum::http::StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(60),
                )),
        )
        .layer(
            CorsLayer::new()
                .allow_methods(cors::AllowMethods::mirror_request())
                .allow_origin(cors::AllowOrigin::mirror_request()),
        )
}

async fn handle_any_root() -> &'static str {
    "NOMS API"
}

async fn handle_healthz() -> &'static str {
    "OK"
}
