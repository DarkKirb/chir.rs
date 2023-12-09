//! Session microservice protocol
//!
//! The session microservice is responsible for validating session tokens, as well as issuing new tokens.
//! It is also responsible for managing scopes and sessions on behalf of other services.

/// Supported scopes
///
/// Each operation uses a different scope from this list, or potentially multiple.
#[non_exhaustive]
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
#[archive(compare(PartialEq))]
pub enum Scopes {
    /// Session management permission
    ///
    /// A session token with this scope can:
    ///
    /// - List active sessions
    /// - Terminate active sessions
    /// - Revoke scopes from active sessions
    ManageSessions,
}

/// Session service
#[tarpc::service]
pub trait SessionService {
    /// Validates a session
    ///
    /// # Arguments
    ///
    /// - `token` - The session token to validate
    /// - `required_scopes` - The required scopes the token must possess
    ///
    /// # Return value
    ///
    /// Returns true if the token is valid and has the required scopes, false otherwise.
    async fn validate_session(token: String, required_scopes: Vec<Scopes>) -> bool;

    /// Removes scopes from a session
    ///
    /// After this call completes, the session will no longer have access to the specified scopes.
    ///
    /// If the session no longer has access to any scopes, it will be invalidated.
    ///
    /// # Arguments
    ///
    /// - `token` - The session token to remove scopes from
    /// - `scopes` - Scopes to revoke access to
    async fn revoke_scopes(token: String, scopes: Vec<Scopes>);

    /// Invalidate token
    ///
    /// After this call completes, the token will no longer be valid, meaning that any subsequent calls to [`validate_session`] will fail.
    ///
    /// [`validate_session`]: SessionService::validate_session
    async fn invalidate_session(token: String);

    /// Lists scopes for the session
    ///
    /// # Arguments
    ///
    /// - `token` - The session token to list scopes for
    ///
    /// # Return value
    ///
    /// List of granted scopes or an empty list if the token is invalid.
    async fn list_scopes(token: String) -> Vec<Scopes>;

    /// List all sessions for current user
    ///
    /// # Arguments
    /// - `token` - A session token with the [`ManageSessions`] scope
    ///
    /// # Return value
    /// A list of session IDs that correspond with the session start time.
    ///
    /// [`ManageSessions`]: Scopes::ManageSessions
    async fn list_sessions(token: String) -> Vec<u128>;

    /// Lists the scopes for a session
    ///
    /// # Arguments
    /// - `token` - A session token with the [`ManageSessions`] scope
    /// - `session_id` - The ID of the session to list scopes for
    ///
    /// # Return value
    /// A list of granted scopes in arbitrary order; empty if the session does not exist or is not accessible.
    ///
    /// [`ManageSessions`]: Scopes::ManageSessions
    async fn list_session_scopes(token: String, session_id: u128) -> Vec<Scopes>;

    /// Revokes scopes from a session.
    ///
    /// Like with [`SessionService::revoke_scopes`], revoking the last scope will invalidate the session.
    ///
    /// # Arguments
    /// - `token` - A session token with the [`ManageSessions`] scope
    /// - `session_id` - The ID of the session to list scopes for
    /// - `scopes` - Scopes to revoke access to
    ///
    /// [`SessionService::revoke_scopes`]: SessionService::revoke_scopes
    /// [`ManageSessions`]: Scopes::ManageSessions]
    async fn revoke_session_scopes(token: String, session_id: u128, scopes: Vec<Scopes>);

    /// Invalidate a session
    ///
    /// # Arguments
    /// - `token` - A session token with the [`ManageSessions`] scope
    /// - `session_id` - The ID of the session to list scopes for
    ///
    /// [`ManageSessions`]: Scopes::ManageSessions
    async fn invalidate_other_session(token: String, session_id: u128);

    // Internal calls:
    /// Create a new session for a user
    ///
    /// # Prerequisites
    ///
    /// The user has successfully authenticated
    ///
    /// # Arguments
    /// - `user_id` - The ID of the user to create a session for
    /// - `scopes` - Scopes to grant access to
    async fn create_session(user_id: u128, scopes: Vec<Scopes>);
}
