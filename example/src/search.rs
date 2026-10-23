use crate::book::Format;

/// The order in which [`Shelf::search`](crate::Shelf::search) returns books.
///
/// An enum instead of a `descending: bool` parameter, so call sites read
/// `SortOrder::TitleDescending` rather than a bare `true`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[expect(
    clippy::exhaustive_enums,
    reason = "two states that could be a bool; a third order is not planned"
)]
pub enum SortOrder {
    /// A to Z by title.
    #[default]
    TitleAscending,
    /// Z to A by title.
    TitleDescending,
}

/// Filters for [`Shelf::search`](crate::Shelf::search).
///
/// A "bag of options" struct, so it implements `Default` and callers write
/// `SearchOptions { author: Some(..), ..SearchOptions::default() }`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[expect(
    clippy::exhaustive_structs,
    reason = "options struct; callers rely on functional record update syntax"
)]
pub struct SearchOptions {
    /// Only return books by this author.
    pub author: Option<String>,
    /// Only return books in this format.
    pub format: Option<Format>,
    /// The order of the results.
    pub order: SortOrder,
}
