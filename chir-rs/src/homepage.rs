use askama_axum::Template;
use axum::response::{Html, IntoResponse};

use crate::{err::RespResult, lang::Locale, theming::Theme};

#[derive(Clone, Debug, PartialEq, Eq, Template)]
#[template(path = "layout.html")]
struct HomepageTemplate {
    theme: Theme,
    locale: Locale,
    entrypoints: String,
}

fn format_embed(embed_filename: String) -> String {
    if embed_filename.ends_with(".css") {
        format!("<link rel=\"stylesheet\" href=\"/static/{embed_filename}\">")
    } else if embed_filename.ends_with(".js") {
        format!("<script defer src=\"/static/{embed_filename}\"></script>")
    } else {
        panic!("don’t know how to handle this");
    }
}

pub async fn homepage(theme: Theme, locale: Locale) -> RespResult<impl IntoResponse> {
    let homepage = HomepageTemplate {
        theme,
        locale,
        entrypoints: include_str!("../../web/entrypoints")
            .split('\n')
            .map(ToString::to_string)
            .filter(|s| !s.is_empty())
            .map(format_embed)
            .fold(String::new(), |mut acc, x| {
                acc += &x;
                acc
            }),
    };
    Ok(Html(homepage.render()?))
}
