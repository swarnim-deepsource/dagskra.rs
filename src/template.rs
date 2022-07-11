use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use super::ruv::{get_shows, Shows, Status};

const AUTHOR: &str = "Paul Burt";
const EMAIL: &str = "paul.burt@bbc.co.uk";
const TITLE: &str = "Dagskrá RÚV";

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    author: &'static str,
    email: &'static str,
    title: &'static str,
    today: String,
}

pub async fn index() -> impl IntoResponse {
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

pub async fn schedule() -> impl IntoResponse {
    let shows = get_shows().await.unwrap_or_default();
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
