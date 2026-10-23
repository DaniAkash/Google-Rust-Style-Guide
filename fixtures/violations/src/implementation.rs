//! Implementation concerns.

use std::fmt::Write;

/// Checks for a value, then unwraps it.
///
/// # Panics
///
/// Never, because of the `is_some` check, but the compiler cannot tell.
pub fn print_value(value: Option<u32>) {
    if value.is_some() {
        let _ = value.unwrap(); //~ clippy::unnecessary_unwrap clippy::unwrap_used
    }
}

/// Asserts without explaining why the condition must hold.
///
/// # Panics
///
/// Panics if `values` is not sorted.
pub fn check_sorted(values: &[u32]) {
    assert!(values.is_sorted()); //~ clippy::missing_assert_message
}

/// Uses `for_each` where a `for` loop reads better.
pub fn print_all(values: &[u32]) {
    values.iter().for_each(|value| {
        //~^ clippy::needless_for_each
        let _ = value;
    });
}

/// Builds a string by pushing `format!` results.
#[must_use]
pub fn join(values: &[u32]) -> String {
    let mut text = String::new();
    for value in values {
        text += &format!("{value},"); //~ clippy::format_push_string
    }
    let _ = write!(text, "end");
    text
}

/// Silences a lint without saying why.
#[allow(clippy::needless_return)] //~ clippy::allow_attributes_without_reason
#[must_use]
pub fn silenced() -> u32 {
    return 1;
}
