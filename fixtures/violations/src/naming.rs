//! Naming.

/// A wrapper around a byte buffer.
#[derive(Debug)]
#[non_exhaustive]
pub struct Buffer {
    bytes: Vec<u8>,
}

impl Buffer {
    /// A constructor named `new` that does not return `Self`.
    #[must_use]
    pub fn new() -> Vec<u8> {
        //~^ clippy::new_ret_no_self
        Vec::new()
    }

    /// An `into_` conversion that only borrows `self`.
    #[must_use]
    pub fn into_bytes(&self) -> Vec<u8> {
        //~^ clippy::wrong_self_convention
        self.bytes.clone()
    }

    /// A length without the matching `is_empty`.
    #[must_use]
    pub fn len(&self) -> usize {
        //~^ clippy::len_without_is_empty
        self.bytes.len()
    }
}

/// Does nothing; other modules call it.
pub fn touch() {}
