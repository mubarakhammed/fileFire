use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tower::Layer;
use std::time::Instant;

pub fn request_logging<S>() -> tower::layer::LayerFn<fn(tower_http::trace::TraceLayer<S>) -> tower_http::trace::TraceLayer<S>>
where 
    S: Clone,
{
    tower::layer::layer_fn(|inner| {
        tower_http::trace::TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                tracing::info_span!(
                    "request",
                    method = %request.method(),
                    uri = %request.uri(),
                )
            })
            .on_request(|_request: &Request<_>, _span: &tracing::Span| {
                tracing::info!("started processing request");
            })
            .on_response(|_response: &Response, latency: std::time::Duration, _span: &tracing::Span| {
                tracing::info!("finished processing request in {:?}", latency);
            })
    })
}

pub async fn logging_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    log::info!(
        "{} {} {} - {:?}",
        method,
        uri,
        status.as_u16(),
        duration
    );
    
    Ok(response)
}