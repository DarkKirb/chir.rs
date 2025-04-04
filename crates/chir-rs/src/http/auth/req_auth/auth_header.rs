//! Authentication header handler

use std::collections::HashSet;

use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use chir_rs_common::http_api::{auth::Scope, errors::APIError};
use eyre::{Context as _, OptionExt as _};
use rusty_paseto::core::{Local, V4};
use rusty_paseto::prelude::PasetoParser;
use tracing::{error, info};

use crate::{db::session::fetch_session_info, http::AppState};

/// Read Authorization from the bearer token.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthHeader(pub String, pub HashSet<Scope>);

impl AuthHeader {
    /// Checks whether or not a scope is granted for the session
    ///
    /// # Errors
    /// This function returns an error if the current session does not have a scope granted.
    pub fn assert_scope(&self, scope: Scope) -> Result<(), APIError> {
        if self.1.contains(&Scope::Full) {
            return Ok(());
        }
        if self.1.contains(&scope) {
            Ok(())
        } else {
            Err(APIError::MissingScope(scope))
        }
    }
}

impl FromRequestParts<AppState> for AuthHeader {
    type Rejection = APIError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Some(authorization_header) = parts.headers.get(AUTHORIZATION) else {
            return Err(APIError::MissingAuthorizationHeader);
        };
        let authorization_header = authorization_header
            .to_str()
            .context("Parsing the authorization header")
            .map_err(|e| APIError::InvalidAuthorizationHeader(format!("{e:?}")))?;

        let Some((method, key)) = authorization_header.split_once(' ') else {
            return Err(APIError::InvalidAuthorizationHeader(
                authorization_header.to_string(),
            ));
        };

        if !method.trim().eq_ignore_ascii_case("Bearer") {
            return Err(APIError::InvalidAuthorizationMethod(
                method.trim().to_string(),
                "Bearer".to_string(),
            ));
        }

        let json = PasetoParser::<V4, Local>::default()
            .parse(key.trim(), &state.paseto_key)
            .context("Verifying paseto token")
            .map_err(|e| {
                info!("Failed authentication with: {e:?}");
                APIError::Unauthorized
            })?;

        let session_id: u128 = json["jti"]
            .as_str()
            .ok_or_eyre("Reading the token ID as a string")
            .and_then(|v| v.parse().context("Parsing session ID"))
            .map_err(|e| {
                error!("Invalid issued token: {e:?}");
                APIError::Unknown(format!("Invalid issued token: {e:?}"))
            })?;

        let session_info = fetch_session_info(&state.db, session_id)
            .await
            .with_context(|| format!("Verifying session {session_id}"))
            .map_err(|e| {
                error!("Failed to verify session: {e:?}");
                APIError::Unknown(format!("Failed to verify session: {e:?}"))
            })?
            .ok_or_eyre("Found session info")
            .map_err(|e| {
                info!("User Error validating session {e:?}");
                APIError::InvalidSession
            })?;
        Ok(Self(session_info.0, session_info.1))
    }
}
