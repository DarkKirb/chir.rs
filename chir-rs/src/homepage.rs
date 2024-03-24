//! Code for the homepage site

use std::path::Path;

use anyhow::Result;
use askama_axum::Template;
use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::{AppendHeaders, Html, IntoResponse},
};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use tracing::instrument;

use crate::{err::RespResult, lang::Locale, theming::Theme};

#[derive(Clone, Debug, PartialEq, Eq, Template)]
#[template(path = "layout.html")]
/// Homepage template variables
struct HomepageTemplate {
    /// Theme used by the homepage
    theme: Theme,
    /// Homepage locale
    locale: Locale,
    /// List of files that need to be loaded on startup according to webpack
    entrypoints: String,
}

/// Format program entrypoint as HTML
fn format_entrypoint(embed_filename: impl AsRef<str>) -> String {
    let embed_filename = embed_filename.as_ref();
    if Path::new(embed_filename)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("css"))
    {
        format!("<link rel=\"stylesheet\" href=\"/static/{embed_filename}\">")
    } else if Path::new(embed_filename)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("js"))
    {
        format!("<script defer src=\"/static/{embed_filename}\"></script>")
    } else {
        unreachable!();
    }
}

/// Render the homepage
///
/// # Errors
/// Returns an error if rendering the homepage fails
#[allow(clippy::unused_async)]
pub async fn homepage(theme: Theme, locale: Locale) -> RespResult<impl IntoResponse> {
    let homepage = HomepageTemplate {
        theme,
        locale,
        entrypoints: include_str!("../../web/entrypoints")
            .split('\n')
            .map(ToString::to_string)
            .filter(|s| !s.is_empty())
            .map(format_entrypoint)
            .fold(String::new(), |mut acc, x| {
                acc += &x;
                acc
            }),
    };
    Ok(Html(homepage.render()?))
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
/// Settings for the `update_settings` endpoint
pub struct Settings {
    /// Theme to use
    theme: String,
    /// Language to use
    lang: String,
}

#[instrument]
/// Updates the user’s setting cookies
pub async fn update_settings(
    cookies: Cookies,
    Query(settings): Query<Settings>,
    headers: HeaderMap,
) -> RespResult<impl IntoResponse> {
    #[allow(clippy::missing_docs_in_private_items)]
    fn update_settings(
        cookies: &Cookies,
        settings: Settings,
        headers: &HeaderMap,
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

        Ok((
            StatusCode::FOUND,
            AppendHeaders([("Location", referrer.to_string())]),
        ))
    }
    Ok(update_settings(&cookies, settings, &headers)?)
}
