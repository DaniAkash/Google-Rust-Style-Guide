use std::error::Error;
use std::str::FromStr;
use std::{fmt, mem};

use crate::isbn::{Isbn, IsbnError};

/// How a book is published.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Format {
    /// A hardcover print edition.
    Hardcover,
    /// A paperback print edition.
    Paperback,
    /// A digital edition.
    Ebook,
}

/// A book on a [`Shelf`](crate::Shelf).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Book {
    title: String,
    author: String,
    isbn: Isbn,
    format: Format,
}

/// The reason a line of text could not be parsed into a [`Book`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[must_use]
#[non_exhaustive]
pub enum ParseBookError {
    /// The line does not have exactly four `|`-separated fields.
    WrongFieldCount,
    /// The ISBN field is not a number.
    IsbnNotANumber,
    /// The ISBN field is a number but not a valid ISBN-13.
    InvalidIsbn(IsbnError),
    /// The format field is not one of `hardcover`, `paperback` or `ebook`.
    UnknownFormat,
}

impl Book {
    /// Creates a book from its parts.
    #[must_use]
    pub fn new(title: String, author: String, isbn: Isbn, format: Format) -> Self {
        Self {
            title,
            author,
            isbn,
            format,
        }
    }

    /// The book's title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// The book's author.
    #[must_use]
    pub fn author(&self) -> &str {
        &self.author
    }

    /// The book's ISBN.
    #[must_use]
    pub fn isbn(&self) -> Isbn {
        self.isbn
    }

    /// The book's format.
    #[must_use]
    pub fn format(&self) -> Format {
        self.format
    }

    /// Changes the title.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Changes the format and returns the previous one.
    pub fn replace_format(&mut self, format: Format) -> Format {
        mem::replace(&mut self.format, format)
    }
}

/// Parses `title|author|isbn|format`, for example
/// `Dune|Frank Herbert|9780441172719|paperback`.
impl FromStr for Book {
    type Err = ParseBookError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = line.split('|').map(str::trim).collect();
        let [title, author, isbn, format] = fields[..] else {
            return Err(ParseBookError::WrongFieldCount);
        };
        let digits: u64 = isbn.parse().map_err(|_| ParseBookError::IsbnNotANumber)?;
        let isbn = Isbn::try_new(digits).map_err(ParseBookError::InvalidIsbn)?;
        let format: Format = format.parse()?;
        Ok(Self::new(title.to_owned(), author.to_owned(), isbn, format))
    }
}

impl Format {
    /// The lowercase name used in the `title|author|isbn|format` text form.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hardcover => "hardcover",
            Self::Paperback => "paperback",
            Self::Ebook => "ebook",
        }
    }
}

impl FromStr for Format {
    type Err = ParseBookError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "hardcover" => Ok(Self::Hardcover),
            "paperback" => Ok(Self::Paperback),
            "ebook" => Ok(Self::Ebook),
            _ => Err(ParseBookError::UnknownFormat),
        }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} by {} ({})",
            self.title, self.author, self.isbn
        )
    }
}

impl fmt::Display for ParseBookError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongFieldCount => formatter.write_str("expected `title|author|isbn|format`"),
            Self::IsbnNotANumber => formatter.write_str("the ISBN is not a number"),
            Self::InvalidIsbn(error) => write!(formatter, "invalid ISBN: {error}"),
            Self::UnknownFormat => formatter.write_str("unknown format"),
        }
    }
}

impl Error for ParseBookError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_line() {
        let book: Book = "Dune | Frank Herbert | 9780441172719 | paperback"
            .parse()
            .unwrap();
        assert_eq!(book.title(), "Dune");
        assert_eq!(book.format(), Format::Paperback);
    }

    #[test]
    fn rejects_missing_fields() {
        let result: Result<Book, ParseBookError> = "Dune|Frank Herbert".parse();
        assert_eq!(result, Err(ParseBookError::WrongFieldCount));
    }
}
