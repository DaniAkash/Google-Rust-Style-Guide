use std::error::Error;
use std::fmt;

/// A 13-digit International Standard Book Number.
///
/// A newtype rather than a `u64` alias, so an ISBN cannot be mixed up with a page count or
/// any other number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Isbn(u64);

/// The reason a value is not a valid ISBN-13.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[must_use]
#[non_exhaustive]
pub enum IsbnError {
    /// The value does not have exactly 13 digits.
    WrongLength,
    /// The final check digit does not match the first 12 digits.
    BadCheckDigit,
}

impl Isbn {
    /// Builds an ISBN from its 13 digits.
    ///
    /// # Errors
    ///
    /// Returns [`IsbnError::WrongLength`] when `digits` is not a 13-digit number and
    /// [`IsbnError::BadCheckDigit`] when its check digit is wrong.
    pub fn try_new(digits: u64) -> Result<Self, IsbnError> {
        if !(1_000_000_000_000..=9_999_999_999_999).contains(&digits) {
            return Err(IsbnError::WrongLength);
        }
        let check_digit = digits % 10;
        if check_digit != expected_check_digit(digits / 10) {
            return Err(IsbnError::BadCheckDigit);
        }
        Ok(Self(digits))
    }

    /// The 13 digits as a number.
    #[must_use]
    pub fn digits(self) -> u64 {
        self.0
    }
}

impl TryFrom<u64> for Isbn {
    type Error = IsbnError;

    fn try_from(digits: u64) -> Result<Self, Self::Error> {
        Self::try_new(digits)
    }
}

impl fmt::Display for Isbn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:013}", self.0)
    }
}

impl fmt::Display for IsbnError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::WrongLength => "an ISBN-13 has exactly 13 digits",
            Self::BadCheckDigit => "the ISBN-13 check digit does not match",
        };
        formatter.write_str(message)
    }
}

impl Error for IsbnError {}

/// ISBN-13 weights the first twelve digits alternately by 1 and 3.
fn expected_check_digit(first_twelve: u64) -> u64 {
    let mut remaining = first_twelve;
    let mut sum = 0;
    for position in 0..12 {
        let digit = remaining % 10;
        remaining /= 10;
        let weight = if position % 2 == 0 { 3 } else { 1 };
        sum += digit * weight;
    }
    (10 - sum % 10) % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_a_valid_isbn() {
        let isbn = Isbn::try_new(9_780_306_406_157).unwrap();
        assert_eq!(isbn.to_string(), "9780306406157");
    }

    #[test]
    fn rejects_a_wrong_check_digit() {
        assert_eq!(
            Isbn::try_new(9_780_306_406_158),
            Err(IsbnError::BadCheckDigit)
        );
    }

    #[test]
    fn rejects_short_numbers() {
        assert_eq!(Isbn::try_new(12_345), Err(IsbnError::WrongLength));
    }
}
