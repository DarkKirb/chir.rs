use askama_axum::Template;
use axum::response::{Html, IntoResponse};

use crate::{err::RespResult, lang::Locale, theming::Theme};

#[derive(Clone, Debug, PartialEq, Eq, Template)]
#[template(path = "layout.html")]
struct HomepageTemplate {
    theme: Theme,
    locale: Locale,
}

pub async fn homepage(theme: Theme, locale: Locale) -> RespResult<impl IntoResponse> {
    let homepage = HomepageTemplate { theme, locale };
    Ok(Html(homepage.render()?))
}
