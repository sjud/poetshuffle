use axum::handler::Handler;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, get_service};
use tokio::io;
use tower_http::services::ServeDir;
use tower::layer::Layer;
use tower::ServiceBuilder;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

#[cfg(feature = "test_cdn")]
pub async fn test_cdn() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8001));
    let app = Router::new()
        .fallback(ServiceBuilder::new()
                      .layer(SetResponseHeaderLayer::if_not_present(
                          header::ACCESS_CONTROL_ALLOW_ORIGIN,
                          HeaderValue::from_static("http://127.0.0.1:8000"),
                      ))
                      .service(get_service(
            ServeDir::new("static"))
            .handle_error(handle_error)
        ))
        .layer(TraceLayer::new_for_http());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server error");

}
async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}