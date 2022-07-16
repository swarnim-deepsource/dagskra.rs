use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use super::show::{get_shows, Shows, Status};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    author: &'static str,
    email: &'static str,
    shows: Shows,
    title: &'static str,
    today: String,
}

pub async fn index() -> impl IntoResponse {
    let shows = get_shows().await.unwrap_or_default();
    let today = chrono::Utc::now().format("%d.%m.%Y").to_string();
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
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
