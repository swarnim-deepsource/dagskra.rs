use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing, Router, Server,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use dagskra_lib::{fetch_schedule, Schedule, Status};

static RUST_LOG: &str = "dagskra_web=debug,tower_http=trace";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var(RUST_LOG).unwrap_or_default()))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
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
    schedule: Schedule,
    title: &'static str,
    today: Option<String>,
}

async fn index() -> impl IntoResponse {
    tracing::debug!("fetching data from external API");
    let schedule = fetch_schedule().await.unwrap_or_default();
    let today = schedule.first().map(|l| l.date());
    let template = IndexTemplate {
        author: "Paul Burt",
        email: "paul.burt@bbc.co.uk",
        schedule,
        title: "Dagskrá RÚV",
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
