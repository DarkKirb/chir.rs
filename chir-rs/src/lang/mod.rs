//! Internationalization support

use std::ops::Deref;

use anyhow::{anyhow, bail};
use anyhow::{Context, Result};
use askama_axum::IntoResponse;
use axum::http::request::Parts;
use axum::http::HeaderMap;
use axum::response::Response;
use axum::{async_trait, extract::FromRequestParts};
use fluent_templates::{static_loader, LanguageIdentifier, Loader};
use phf::{phf_map, Map};
use tower_cookies::Cookies;
use tracing::{instrument, warn};
use unic_langid::langid;

static_loader! {
    static LOCALES = {
        locales: "../locales",
        fallback_language: "en-US"
    };
}

static LANGUAGE_IDS: Map<&'static str, LanguageIdentifier> = phf_map! {
    "de" => langid!("de-DE"),
    "de-DE" => langid!("de-DE"),
    "en" => langid!("en-US"),
    "en-GB" => langid!("en-GB"),
    "en-IE" => langid!("en-GB"),
    "en-Sten-US" => langid!("en-Sten-US"),
    "en-US" => langid!("en-US"),
    "fr" => langid!("fr-FR"),
    "fr-FR" => langid!("fr-FR"),
    "jbo" => langid!("jbo-Latn-XX"),
    "jbo-XX" => langid!("jbo-Latn-XX"),
    "jbo-Latn-XX" => langid!("jbo-Latn-XX"),
    "jbo-Zblr-XX" => langid!("jbo-Zblr-XX"),
    "nl" => langid!("nl-NL"),
    "nl-NL" => langid!("nl-NL"),
    "tok" => langid!("tok-Latn-XX"),
    "tok-XX" => langid!("tok-Latn-XX"),
    "tok-Emsi-XX" => langid!("tok-Emsi-XX"),
    "tok-Hani-XX" => langid!("tok-Hani-XX"),
    "tok-Latn-XX" => langid!("tok-Latn-XX"),
    "tok-Stln-XX" => langid!("tok-Stln-XX"),
};
static FALLBACK_LANG: LanguageIdentifier = langid!("en");

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Locale(Vec<&'static LanguageIdentifier>);

impl Locale {
    fn determine_known_locale(locale: impl AsRef<str>) -> Result<&'static LanguageIdentifier> {
        let locale = locale.as_ref();

        // See if we know about the locale directly
        if let Some(lang) = LANGUAGE_IDS.get(locale) {
            return Ok(lang);
        }

        let locale: LanguageIdentifier = locale
            .parse()
            .with_context(|| format!("Parsing locale: {locale}"))?;

        let (language, script, region, mut variants) = locale.clone().into_parts();

        // Remove variants one-by-one until we find a match
        while !variants.is_empty() {
            variants.pop();
            let lang_id = LanguageIdentifier::from_parts(
                language.clone(),
                script.clone(),
                region.clone(),
                &variants,
            )
            .to_string();

            if let Some(lang) = LANGUAGE_IDS.get(&lang_id) {
                return Ok(lang);
            }
        }

        // Remove script tag
        let lang_id =
            LanguageIdentifier::from_parts(language.clone(), None, region.clone(), &[]).to_string();

        if let Some(lang) = LANGUAGE_IDS.get(&lang_id) {
            return Ok(lang);
        }

        // Remove region tag
        let lang_id = LanguageIdentifier::from_parts(language.clone(), None, None, &[]).to_string();

        if let Some(lang) = LANGUAGE_IDS.get(&lang_id) {
            return Ok(lang);
        }

        bail!("Unknown locale: {locale}");
    }
    pub fn new(http_header: impl AsRef<str>) -> Self {
        let mut langs = accept_language::parse(http_header.as_ref())
            .iter()
            .filter_map(|v| match Locale::determine_known_locale(v) {
                Ok(lang) => Some(lang),
                Err(e) => {
                    warn!("Unknown language: {v} ({e:?})");
                    None
                }
            })
            .collect::<Vec<_>>();
        if !langs.contains(&&FALLBACK_LANG) {
            langs.push(&FALLBACK_LANG);
        }
        Self(langs)
    }

    pub fn prepend_language(&mut self, lang: impl AsRef<str>) {
        let lang_id = match LANGUAGE_IDS.get(lang.as_ref()) {
            Some(lang_id) => lang_id,
            None => {
                return;
            }
        };
        self.0.insert(0, lang_id);
        self.0.dedup();
    }
    pub fn append_language(&mut self, lang: impl AsRef<str>) {
        let lang_id = match LANGUAGE_IDS.get(lang.as_ref()) {
            Some(lang_id) => lang_id,
            None => {
                return;
            }
        };
        if !self.0.contains(&lang_id) {
            self.0.push(lang_id);
        }
    }
    pub fn trans(&self, key: impl AsRef<str>) -> String {
        let text_id = key.as_ref();
        for lang in &self.0 {
            if let Some(translation) = LOCALES.try_lookup(lang, text_id) {
                return translation;
            }
        }
        if let Some(translation) = LOCALES.try_lookup(&FALLBACK_LANG, text_id) {
            return translation;
        }
        return text_id.to_string();
    }

    pub fn preferred_language(&self) -> String {
        self.0
            .get(0)
            .map(Deref::deref)
            .unwrap_or(&FALLBACK_LANG)
            .to_string()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Locale
where
    S: Send + Sync,
{
    type Rejection = Response;
    #[instrument(skip(state))]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        async fn from_request_parts<S: Send + Sync>(
            parts: &mut Parts,
            state: &S,
        ) -> Result<Locale> {
            let headers = HeaderMap::from_request_parts(parts, state)
                .await
                .context("Extracting headers from request")?;
            let cookies = Cookies::from_request_parts(parts, state)
                .await
                .map_err(|e| anyhow!("Could not extract cookies from request: {e:?}"))?;
            let header = headers
                .get("Accept-Language")
                .ok_or_else(|| anyhow!("Missing Accept-Language header"))
                .and_then(|v| Ok(v.to_str()?))
                .unwrap_or("en-US");
            let mut locale = Locale::new(header);
            if let Some(cookie) = cookies.get("locale") {
                locale.prepend_language(cookie.value());
            }
            locale.append_language("en-us");
            Ok(locale)
        }
        from_request_parts(parts, state).await.map_err(|e| {
            let e = crate::err::Error::from(e);
            e.into_response()
        })
    }
}
