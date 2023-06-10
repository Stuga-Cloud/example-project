use axum::{http::header, Router};

use crate::{logger, routes};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer, trace,
};

pub async fn create_app() -> Router {
    logger::setup();

    Router::new()
        .merge(routes::status::create_route())
        .merge(Router::new().nest(
            "/v1",
            // All public v1 routes will be nested here.
            Router::new().merge(routes::product::create_route()),
        ))
        // High level logging of requests and responses
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        // Mark the `Authorization` request header as sensitive so it doesn't
        // show in logs.
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(
            header::AUTHORIZATION,
        )))
        // Compress responses
        .layer(CompressionLayer::new())
        // Propagate `X-Request-Id`s from requests to responses
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
            "x-request-id",
        )))
        // CORS configuration. This should probably be more restrictive in
        // production.
        .layer(CorsLayer::permissive())
}
