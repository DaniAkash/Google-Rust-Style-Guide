//! A small bookshelf library written to the Google Rust style guide.
//!
//! The crate root is a non-leaf module, so it only declares submodules and re-exports their
//! public items. Every API lives in a leaf module named after its primary type.

mod book;
mod isbn;
mod search;
mod shelf;

pub use book::{Book, Format, ParseBookError};
pub use isbn::{Isbn, IsbnError};
pub use search::{SearchOptions, SortOrder};
pub use shelf::Shelf;
