//! Traits.

use std::fmt;

/// A temperature in degrees Celsius.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Celsius {
    degrees: i32,
}

impl Celsius {
    /// Creates a temperature at zero degrees.
    #[must_use]
    pub fn new() -> Self {
        //~^ clippy::new_without_default
        Self { degrees: 0 }
    }

    /// Parses a temperature with a custom method instead of `FromStr`.
    #[expect(
        clippy::missing_errors_doc,
        reason = "fixture focuses on should_implement_trait"
    )]
    pub fn from_str(text: &str) -> Result<Self, std::num::ParseIntError> {
        //~^ clippy::should_implement_trait clippy::absolute_paths
        Ok(Self {
            degrees: text.parse()?,
        })
    }

    /// Formats with a custom method instead of `Display`.
    #[must_use]
    pub fn to_string(&self) -> String {
        //~^ clippy::inherent_to_string
        format!("{}°C", self.degrees)
    }
}

/// A distance in meters.
#[derive(Debug)]
#[non_exhaustive]
pub struct Meters {
    value: u32,
}

impl Into<u32> for Meters {
    //~^ clippy::from_over_into
    fn into(self) -> u32 {
        self.value
    }
}

/// A counter with a hand-written `Default`.
#[derive(Debug)]
#[non_exhaustive]
pub struct Counter {
    count: u32,
}

impl Default for Counter {
    //~^ clippy::derivable_impls
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.count)
    }
}

/// A point with a hand-written `Clone` on a `Copy` type.
#[derive(Debug, Copy)]
#[non_exhaustive]
pub struct Point {
    x: i32,
}

impl Clone for Point {
    //~^ clippy::expl_impl_clone_on_copy
    fn clone(&self) -> Self {
        //~^ clippy::non_canonical_clone_impl
        Self { x: self.x }
    }
}
