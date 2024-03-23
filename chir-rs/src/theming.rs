use std::{borrow::Cow, fmt::Display};

use anyhow::{anyhow, Context, Result};
use axum::{
    async_trait, extract::FromRequestParts, http::request::Parts, response::Response,
    RequestPartsExt,
};
use phf::{phf_map, Map};
use tower_cookies::{Cookie, Cookies};
use tracing::{instrument, warn};

/// Themes for the chir.rs website
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Theme {
    /// Automatic theme selection, based on browser settings
    Auto,
    /// Dark theme with a red-purple-black background
    Sunset,
    /// Light theme with a blue-pink-white background
    TransRights,
    /// High contrast dark theme
    Black,
    /// High contrast light theme
    White,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Auto
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Sunset => write!(f, "sunset"),
            Self::TransRights => write!(f, "trans-rights"),
            Self::Black => write!(f, "black"),
            Self::White => write!(f, "white"),
        }
    }
}

impl TryFrom<&str> for Theme {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        static THEMES: Map<&'static str, Theme> = phf_map! {
            "auto" => Theme::Auto,
            "sunset" => Theme::Sunset,
            "trans-rights" => Theme::TransRights,
            "black" => Theme::Black,
            "white" => Theme::White,
        };
        if let Some(value) = THEMES.get(value) {
            return Ok(*value);
        } else {
            return Err(anyhow!("Invalid theme value: {}", value));
        }
    }
}

impl Theme {
    pub fn is_selected(&self, theme: Theme) -> &'static str {
            if *self == theme {
                "selected"
            } else {
                ""
            }
        
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Theme
where
    S: Send + Sync,
{
    type Rejection = Response;
    #[instrument(skip(_state))]
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookies = match parts.extract::<Cookies>().await {
            Ok(cookies) => cookies,
            Err(e) => {
                warn!("Could not extract cookies from request: {e:?}");
                return Ok(Theme::Auto);
            }
        };
        let theme_cookie = cookies
            .get("theme")
            .map(|v| Cow::Owned(v.value().to_owned()))
            .unwrap_or(Cow::Borrowed("auto"));
        Theme::try_from(theme_cookie.as_ref())
            .context("While reading theme cookie")
            .or_else(|e| {
                warn!("Invalid theme cookie value: {e:?}. Resetting.");
                cookies.remove(Cookie::new("theme", ""));
                Ok(Theme::Auto)
            })
    }
}
