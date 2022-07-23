use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing, Router, Server,
};
use axum_extra::routing::SpaRouter;
use std::{env, net::SocketAddr};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use dagskra_lib::{get_shows, Shows, Status};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "axum_ruv=debug,tower_http=trace".into()),
        ))
        .with(fmt::layer())
        .init();
    let assets = SpaRouter::new("/static", "./assets");
    let app = Router::new()
        .merge(assets)
        .route("/", routing::get(index))
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    author: &'static str,
    email: &'static str,
    shows: Shows,
    title: &'static str,
    today: String,
}

async fn index() -> impl IntoResponse {
    let shows = get_shows().await.unwrap_or_default();
    let today = shows.first().map_or("".into(), |s| s.date());
    let template = IndexTemplate {
        author: "Paul Burt",
        email: "paul.burt@bbc.co.uk",
        title: "Dagskrá RÚV",
        shows,
        today,
    };
    HtmlTemplate(template)
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}
