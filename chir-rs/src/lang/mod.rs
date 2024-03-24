//! Internationalization support

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
use tracing::{instrument, trace, warn};
use unic_langid::langid;

static_loader! {
    static LOCALES = {
        locales: "../locales",
        fallback_language: "en-US"
    };
}

/// List of known language IDs
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
/// Default language that is loaded if there is no valid other language
static FALLBACK_LANG: LanguageIdentifier = langid!("en");

#[derive(Clone, Debug)]
/// Identifies a locale, optionally with the user-defined original spelling
struct LocaleID {
    /// Known language identifier
    language_id: &'static LanguageIdentifier,
    /// Original spelling of the locale
    display_language_id: Option<String>,
}

impl LocaleID {
    /// Returns the original spelling of the language code
    fn language_code(&self) -> String {
        self.display_language_id
            .clone()
            .unwrap_or_else(|| self.language_id.to_string())
    }
}

impl From<&'static LanguageIdentifier> for LocaleID {
    fn from(language_id: &'static LanguageIdentifier) -> Self {
        Self {
            language_id,
            display_language_id: None,
        }
    }
}

impl PartialEq for LocaleID {
    fn eq(&self, other: &Self) -> bool {
        self.language_code() == other.language_code()
    }
}

impl Eq for LocaleID {}

#[derive(Clone, Debug, PartialEq, Eq)]
/// User-selected locale
pub struct Locale {
    /// List of known locales, in order of user preference
    locale_preference: Vec<LocaleID>,
    /// User-set locale
    set_locale: Option<String>,
}

impl Locale {
    /// Attempts to parse a locale from a string
    ///
    /// # Errors
    /// This function returns an error if the locale is not valid or unknown
    #[allow(clippy::cognitive_complexity)]
    fn determine_known_locale(locale: impl AsRef<str>) -> Result<LocaleID> {
        let locale = locale.as_ref();
        trace!("Determining known locale for {locale}");

        // See if we know about the locale directly
        if let Some(lang) = LANGUAGE_IDS.get(locale) {
            trace!("We know about the locale {locale} directly");
            return Ok(lang.into());
        }

        let locale: LanguageIdentifier = locale
            .parse()
            .with_context(|| format!("Parsing locale: {locale}"))?;

        trace!("Parsed locale {locale}");

        let (language, script, region, mut variants) = locale.clone().into_parts();

        trace!("{locale} has language: {language}, script: {script:?}, region: {region:?}, variants: {variants:?}");

        // Remove variants one-by-one until we find a match
        while !variants.is_empty() {
            variants.pop();
            let lang_id =
                LanguageIdentifier::from_parts(language, script, region, &variants).to_string();

            if let Some(lang) = LANGUAGE_IDS.get(&lang_id) {
                return Ok(LocaleID {
                    language_id: lang,
                    display_language_id: Some(locale.to_string()),
                });
            }
        }

        // Remove script tag
        let lang_id = LanguageIdentifier::from_parts(language, None, region, &[]).to_string();

        if let Some(lang) = LANGUAGE_IDS.get(&lang_id) {
            return Ok(LocaleID {
                language_id: lang,
                display_language_id: Some(locale.to_string()),
            });
        }

        // Remove region tag
        let lang_id = LanguageIdentifier::from_parts(language, None, None, &[]).to_string();

        if let Some(lang) = LANGUAGE_IDS.get(&lang_id) {
            return Ok(LocaleID {
                language_id: lang,
                display_language_id: Some(locale.to_string()),
            });
        }

        bail!("Unknown locale: {locale}");
    }

    /// Creates a new Locale from the value of the HTTP `Accept-Language` header.
    pub fn new(http_header: impl AsRef<str>) -> Self {
        let langs = accept_language::parse(http_header.as_ref())
            .iter()
            .filter_map(|v| match Self::determine_known_locale(v) {
                Ok(lang) => Some(lang),
                Err(e) => {
                    warn!("Unknown language: {v} ({e:?})");
                    None
                }
            })
            .collect::<Vec<_>>();
        Self {
            locale_preference: langs,
            set_locale: None,
        }
    }

    /// Prepends a language to the beginning of the locale preference list
    pub fn prepend_language(&mut self, lang: impl AsRef<str>) {
        let lang = lang.as_ref();
        let lang_id = match Self::determine_known_locale(lang) {
            Ok(lang_id) => lang_id,
            Err(e) => {
                warn!("Failed to parse locale {lang}: {e:?}");
                return;
            }
        };
        self.locale_preference.insert(0, lang_id);
        self.locale_preference.dedup();
        self.set_locale = Some(lang.to_string());
    }

    /// Appends a language to the end of the locale preference list
    pub fn append_language(&mut self, lang: impl AsRef<str>) {
        let lang = lang.as_ref();
        let lang_id = match Self::determine_known_locale(lang) {
            Ok(lang_id) => lang_id,
            Err(e) => {
                warn!("Failed to parse locale {lang}: {e:?}");
                return;
            }
        };
        if !self.locale_preference.contains(&lang_id) {
            self.locale_preference.push(lang_id);
        }
    }

    /// Looks up a translation key in the current locale
    pub fn trans(&self, key: impl AsRef<str>) -> String {
        let text_id = key.as_ref();
        for lang in &self.locale_preference {
            if let Some(translation) = LOCALES.try_lookup(lang.language_id, text_id) {
                return translation;
            }
        }
        if let Some(translation) = LOCALES.try_lookup(&FALLBACK_LANG, text_id) {
            return translation;
        }
        text_id.to_string()
    }

    /// Returns the preferred language of the user
    pub fn preferred_language(&self) -> String {
        self.locale_preference
            .first()
            .map_or_else(|| FALLBACK_LANG.to_string(), LocaleID::language_code)
    }

    /// Returns `"selected"` if the given language is the user-selected language
    pub fn is_selected_language(&self, lang: impl AsRef<str>) -> &'static str {
        let lang = lang.as_ref();

        if self.set_locale.as_ref().map_or("auto", String::as_str) == lang {
            "selected"
        } else {
            ""
        }
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
        #[allow(clippy::missing_docs_in_private_items)]
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
