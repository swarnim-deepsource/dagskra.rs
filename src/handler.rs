mod show;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use self::show::{get_shows, Shows, Status};

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
    let today = shows.first().map_or(String::from(""), |s| s.date());
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
