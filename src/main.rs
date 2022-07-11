mod ruv;

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

use ruv::{Shows, Status};

const AUTHOR: &str = "Paul Burt";
const EMAIL: &str = "paul.burt@bbc.co.uk";
const TITLE: &str = "Dagskrá RÚV";

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
        .route("/", routing::get(index))
        .route("/_schedule", routing::get(schedule))
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("Listening on {}", addr);
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
    title: &'static str,
    today: String,
}

async fn index() -> impl IntoResponse {
    let today = chrono::Utc::now().format("%d.%m.%Y").to_string();
    let template = IndexTemplate {
        author: AUTHOR,
        email: EMAIL,
        title: TITLE,
        today,
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "schedule.html")]
struct ScheduleTemplate {
    shows: Shows,
}

async fn schedule() -> impl IntoResponse {
    let shows = ruv::get_shows().await.unwrap();
    let template = ScheduleTemplate { shows };
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
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
