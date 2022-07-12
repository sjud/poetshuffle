use axum::http::{StatusCode};
use axum::response::IntoResponse;
use tokio::io;
use tower_http::cors::CorsLayer;

#[cfg(feature = "local_cdn")]
pub async fn local_cdn() {
    use tower::ServiceBuilder;
    use tower_http::services::ServeDir;
    use tower_http::trace::TraceLayer;
    use tower_http::set_header::SetResponseHeaderLayer;
    use axum::Router;
    use axum::routing::get_service;
    use axum::http::{header, HeaderValue};

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8001));
    let app = Router::new()
        .fallback(
            ServiceBuilder::new()
                .layer(CorsLayer::very_permissive())
                .service(get_service(ServeDir::new("static"))
                    .handle_error(handle_error)),
        )
        .layer(TraceLayer::new_for_http());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server error");
}
#[cfg(feature = "local_cdn")]
async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}