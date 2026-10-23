//! Ownership and borrowing.

use std::rc::Rc; //~ clippy::disallowed_types

/// Shares a value through reference counting.
#[must_use]
pub fn share(value: u8) -> Rc<u8> {
    //~^ clippy::disallowed_types
    Rc::new(value) //~ clippy::disallowed_types
}

/// Takes a `&String` where `&str` would accept more callers.
#[must_use]
pub fn name_len(name: &String) -> usize {
    //~^ clippy::ptr_arg
    name.len()
}

/// Clones a string whose original is never used again.
#[must_use]
pub fn shout(name: &str) -> String {
    let original = name.to_uppercase();
    let copy = original.clone(); //~ clippy::redundant_clone
    copy
}

/// Copies a string through `to_string` instead of `clone`.
#[must_use]
pub fn duplicate(name: &str) -> usize {
    let owned = name.to_owned();
    let again = owned.to_string(); //~ clippy::implicit_clone
    owned.len() + again.len()
}

/// Takes ownership of a vector it only reads.
#[must_use]
pub fn total(values: Vec<u32>) -> u32 {
    //~^ clippy::needless_pass_by_value
    values.iter().sum()
}
