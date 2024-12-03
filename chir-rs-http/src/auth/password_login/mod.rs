//! Password based login flow

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::extract::State;
use chir_rs_db::user::User;
use chir_rs_http_api::{
    auth::{LoginRequest, PasetoToken},
    axum::bincode::Bincode,
    errors::APIError,
};
use chrono::{Days, Utc};
use eyre::{Context as _, OptionExt};
use rusty_paseto::{
    core::{Local, V4},
    prelude::{
        AudienceClaim, ExpirationClaim, IssuerClaim, PasetoBuilder, SubjectClaim,
        TokenIdentifierClaim,
    },
};
use tokio::task::spawn_blocking;
use tracing::instrument;
use unicode_normalization::UnicodeNormalization;

use crate::AppState;

/// Logs in using username and password
#[instrument]
pub async fn login(
    State(state): State<AppState>,
    Bincode(login_request): Bincode<LoginRequest>,
) -> Result<Bincode<PasetoToken>, APIError> {
    let Some(user_info) = User::get(&state.db, &login_request.username)
        .await
        .with_context(|| format!("Fetching user info for {}", login_request.username))
        .map_err(|e| APIError::DatabaseError(format!("{e:?}")))?
    else {
        return Err(APIError::UserNotFound(login_request.username));
    };

    let password = login_request.password.trim().nfkc().collect::<String>();

    let pwhash = user_info.password_hash.clone();

    spawn_blocking(move || {
        let pwhash = PasswordHash::new(&pwhash)?;
        Argon2::default().verify_password(password.as_bytes(), &pwhash)
    })
    .await
    .with_context(|| {
        format!(
            "Hashing password for login request for {}",
            login_request.username
        )
    })
    .map_err(|e| APIError::Unknown(format!("{e:?}")))?
    .map_err(|_| APIError::InvalidPassword(login_request.username.clone()))?;

    let session_id = user_info
        .new_session(&state.db, login_request.scopes)
        .await
        .with_context(|| format!("Creating context for {}", login_request.username))
        .map_err(|e| APIError::DatabaseError(format!("{e:?}")))?
        .to_string();

    let now = Utc::now();
    let tomorrow = now
        .checked_add_days(Days::new(1))
        .ok_or_eyre("We should not be running several thousand years in the future!")
        .map_err(|e| APIError::Unknown(format!("{e:?}")))?;

    let token = PasetoBuilder::<V4, Local>::default()
        .set_claim(AudienceClaim::from("https://lotte.chir.rs"))
        .set_claim(SubjectClaim::from(login_request.username.as_str()))
        .set_claim(IssuerClaim::from("https://lotte.chir.rs"))
        .set_claim(TokenIdentifierClaim::from(session_id.as_str()))
        .set_claim(
            ExpirationClaim::try_from(tomorrow.to_rfc3339().as_str())
                .context("valid date")
                .map_err(|e| APIError::Unknown(format!("{e:?}")))?,
        )
        .build(&state.paseto_key)
        .context("Signing paseto key")
        .map_err(|e| APIError::Unknown(format!("{e:?}")))?;

    Ok(Bincode(
        PasetoToken::from_paseto(&token).map_err(|e| APIError::Unknown(format!("{e:?}")))?,
    ))
}
