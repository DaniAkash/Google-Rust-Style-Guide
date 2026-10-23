//! Deliberate violations of the Google Rust style guide.
//!
//! Every line that should trigger a lint carries a marker comment naming that lint, either on
//! the same line or (with a caret) on the next line. `scripts/check-fixtures.sh` fails when a
//! marked lint does not fire or an unmarked lint does, so this crate proves the configuration
//! in the repository root enforces what the docs claim.

pub mod borrowing;
pub mod documentation;
pub mod features;
pub mod implementation;
pub mod inputs;
pub mod naming;
pub mod organization;
pub mod traits;
