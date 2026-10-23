//! Code organization and formatting.

use std::collections::*; //~ clippy::wildcard_imports

/// Builds an empty map.
#[must_use]
pub fn empty_map() -> HashMap<u8, u8> {
    HashMap::new()
}

/// Reads a file through a fully spelled-out path.
pub fn read_config() {
    let _ = std::fs::read("config.toml"); //~ clippy::absolute_paths
}

/// Calls a sibling module's function through `crate::`.
pub fn call_sibling() {
    crate::naming::touch(); //~ clippy::absolute_paths
}

/// A greeting with a zero-width space hidden between the two words.
#[must_use]
pub fn greeting() -> &'static str {
    "hello​world" //~ clippy::invisible_characters
}
