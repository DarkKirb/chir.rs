//! Robots APIs

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Data of a single robots.txt entry
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct RobotsRule {
    /// ID of the rule
    pub id: i32,
    /// Robot to apply the rule to
    pub user_agent: String,
    /// Path to apply the rule to
    pub path: String,
    /// Whether to allow or deny raccess
    pub allow: bool,
}
