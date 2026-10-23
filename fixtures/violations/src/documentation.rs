//! Documentation.

pub fn undocumented() {} //~ missing_docs

/// Returns the first byte.
#[must_use]
pub fn first(bytes: &[u8]) -> u8 {
    //~^ clippy::missing_panics_doc
    *bytes.first().expect("callers pass non-empty slices")
}

/// Parses a number.
pub fn parse(text: &str) -> Result<u32, String> {
    //~^ clippy::missing_errors_doc
    text.parse().map_err(|_| format!("not a number: {text}"))
}

/// Reads the byte behind a raw pointer.
#[must_use]
pub unsafe fn read_raw(pointer: *const u8) -> u8 {
    //~^ clippy::missing_safety_doc
    // SAFETY: the caller guarantees `pointer` is valid for reads.
    unsafe { *pointer }
}

/// Mentions an item name like SomeType without backticks.
pub fn mentions_code() {} //~^ clippy::doc_markdown
