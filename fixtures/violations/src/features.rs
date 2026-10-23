//! Other Rust features.

use std::panic;
use std::sync::Mutex;

/// A public enum that callers can match exhaustively.
#[derive(Debug)]
pub enum Status {
    //~^ clippy::exhaustive_enums
    /// Running.
    Running,
    /// Stopped.
    Stopped,
}

/// A public struct whose fields are all public.
#[derive(Debug)]
pub struct Config {
    //~^ clippy::exhaustive_structs
    /// A name.
    pub name: String,
}

/// A pure function that is not marked `#[must_use]`.
pub fn double(value: u32) -> u32 {
    //~^ clippy::must_use_candidate
    value * 2
}

/// Dereferences a raw pointer with no comment explaining why that is sound.
///
/// # Panics
///
/// Panics if `bytes` is empty.
#[must_use]
pub fn first_byte(bytes: &[u8]) -> u8 {
    assert!(!bytes.is_empty(), "callers pass non-empty slices");
    unsafe { *bytes.as_ptr() } //~ clippy::undocumented_unsafe_blocks
}

/// Adds two numbers, with a stray safety comment.
#[must_use]
pub fn add(left: u32, right: u32) -> u32 {
    // SAFETY: nothing unsafe happens here.
    left.wrapping_add(right) //~ clippy::unnecessary_safety_comment
}

/// Reaches inside a poisoned lock.
#[must_use]
pub fn read_through_poison(lock: &Mutex<u8>) -> u8 {
    *lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) //~ clippy::disallowed_methods
}

/// Catches panics instead of preventing them.
#[must_use]
pub fn swallow_panic() -> bool {
    panic::catch_unwind(|| 1).is_ok() //~ clippy::disallowed_methods
}

/// Raw pointer reads inside an `unsafe fn` without an `unsafe` block.
///
/// # Safety
///
/// `pointer` must be valid for reads.
#[must_use]
pub unsafe fn read_unchecked(pointer: *const u8) -> u8 {
    *pointer //~ E0133
}
