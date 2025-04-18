//! Snowflake-like ID generator

use std::{
    cell::{Cell, LazyCell},
    sync::LazyLock,
};

use chrono::Utc;
use rand::Rng;

/// Generates a Snowflake ID
///
/// These IDs are roughly time-ordered across nodes, assuming that their clocks are reasonably in sync.
///
/// On a single system, the IDs are monotonically iff the system clock is monotonically increasing.
///
/// Due to the high accuracy as well as counters, duplicate IDs are avoided in the case that the clock is rewound.
///
/// # Panics
/// This function will panic if the current system time in UTC is before the start of the unix epoch [aka misconfigured].
#[must_use]
#[expect(
    clippy::expect_used,
    reason = "Only possible with serious misconfiguration of the system"
)]
pub fn generate() -> u128 {
    /// Application-global Node ID
    static NODE_ID: LazyLock<u16> = LazyLock::new(|| rand::rng().random());
    thread_local! {
        /// Random Thread ID
        static THREAD_ID: LazyCell<u16> = LazyCell::new(|| rand::rng().random());
        /// Per-thread Counter
        static THREAD_COUNTER: Cell<u16> = const { Cell::new(0) };
    };

    let now = Utc::now();

    let counter = THREAD_COUNTER.get();
    THREAD_COUNTER.set(counter.wrapping_add(1));

    let mut id: u128 = now.timestamp().try_into().expect("Configured Clock");
    id *= 1_000_000_000; // Adjust for nanoseconds
    id += u128::from(now.timestamp_subsec_nanos());
    id <<= 16;
    id |= u128::from(*NODE_ID);
    id <<= 16;
    id |= u128::from(THREAD_ID.with(|v| **v));
    id <<= 16;
    id |= u128::from(counter);

    id
}

/// Generates a string snowflake ID
///
/// This is appropriate to be inserted into URLs and is ordered like the integer IDs.
///
/// # Panics
/// This function will panic if the current system time in UTC is before the start of the unix epoch [aka misconfigured].
#[must_use]
pub fn generate_string() -> String {
    use crate::lexicographic_base64;

    let id = generate();
    let id_bytes = id.to_be_bytes();
    lexicographic_base64::encode(id_bytes)
}
