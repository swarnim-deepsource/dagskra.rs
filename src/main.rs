mod handler;

use axum::{routing, Router, Server};
use axum_extra::routing::SpaRouter;
use std::{env, net::SocketAddr};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "axum_ruv=debug,tower_http=trace".into()),
        ))
        .with(fmt::layer())
        .init();
    let spa = SpaRouter::new("/static", "./static");
    let app = Router::new()
        .merge(spa)
        .route("/", routing::get(handler::index))
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
