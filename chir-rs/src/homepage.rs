use anyhow::Result;
use askama_axum::Template;
use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::{AppendHeaders, Html, IntoResponse},
};
use tower_cookies::{Cookie, Cookies};
use tracing::instrument;
use serde::Deserialize;

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

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct Settings {
    theme: String,
    lang: String,
}

#[instrument]
pub async fn update_settings(
    cookies: Cookies,
    Query(settings): Query<Settings>,
    headers: HeaderMap,
) -> RespResult<impl IntoResponse> {
    async fn update_settings(
        cookies: Cookies,
        settings: Settings,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse> {
        let referrer = match headers.get("referer") {
            Some(v) => v.to_str()?,
            None => return Err(anyhow::anyhow!("Missing referer header")),
        };

        if settings.theme == "auto" {
            cookies.remove(Cookie::new("theme", ""));
        } else {
            let mut theme_cookie = Cookie::new("theme", settings.theme);
            theme_cookie.make_permanent();
            cookies.add(theme_cookie);
        }

        if settings.lang == "auto" {
            cookies.remove(Cookie::new("locale", ""));
        } else {
            let mut locale_cookie = Cookie::new("locale", settings.lang);
            locale_cookie.make_permanent();
            cookies.add(locale_cookie);
        }
    
        Ok((StatusCode::FOUND, AppendHeaders([("Location", referrer.to_string())])))
    }
    Ok(update_settings(cookies, settings, headers).await?)
}
