use crate::book::{Book, Format};
use crate::isbn::Isbn;
use crate::search::{SearchOptions, SortOrder};

/// An ordered collection of books with unique ISBNs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Shelf {
    books: Vec<Book>,
}

impl Shelf {
    /// Creates an empty shelf.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of books on the shelf.
    #[must_use]
    pub fn len(&self) -> usize {
        self.books.len()
    }

    /// Whether the shelf has no books.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    /// Adds a book.
    ///
    /// # Panics
    ///
    /// Panics if a book with the same ISBN is already on the shelf. Check with
    /// [`Shelf::contains`] first when the input is untrusted.
    pub fn add(&mut self, book: Book) {
        assert!(
            !self.contains(book.isbn()),
            "{} is already on the shelf; call `contains` before `add`",
            book.isbn()
        );
        self.books.push(book);
    }

    /// Whether a book with this ISBN is on the shelf.
    #[must_use]
    pub fn contains(&self, isbn: Isbn) -> bool {
        self.books.iter().any(|book| book.isbn() == isbn)
    }

    /// Removes the book with this ISBN and returns it.
    pub fn take_book(&mut self, isbn: Isbn) -> Option<Book> {
        let index = self.books.iter().position(|book| book.isbn() == isbn)?;
        Some(self.books.remove(index))
    }

    /// Calls `edit` with a mutable borrow of the book with this ISBN, then keeps the shelf
    /// sorted by title. Returns `None` when no such book exists.
    pub fn with_mut_book<R>(&mut self, isbn: Isbn, edit: impl FnOnce(&mut Book) -> R) -> Option<R> {
        let book = self.books.iter_mut().find(|book| book.isbn() == isbn)?;
        let result = edit(book);
        self.books
            .sort_by(|left, right| left.title().cmp(right.title()));
        Some(result)
    }

    /// Books in the given format, in shelf order.
    pub fn books_in(&self, format: Format) -> impl Iterator<Item = &Book> {
        self.books
            .iter()
            .filter(move |book| book.format() == format)
    }

    /// Books matching every filter in `options`, sorted by `options.order`.
    #[must_use]
    pub fn search(&self, options: &SearchOptions) -> Vec<&Book> {
        let candidates = self.books.iter().filter(|book| {
            let author_matches = options
                .author
                .as_deref()
                .is_none_or(|author| book.author() == author);
            let format_matches = options.format.is_none_or(|format| book.format() == format);
            author_matches && format_matches
        });

        let mut matches: Vec<&Book> = Vec::new();
        for book in candidates {
            matches.push(book);
        }
        matches.sort_by(|left, right| match options.order {
            SortOrder::TitleAscending => left.title().cmp(right.title()),
            SortOrder::TitleDescending => right.title().cmp(left.title()),
        });
        matches
    }

    /// One line per book, in `title|author|isbn|format` form.
    #[must_use]
    pub fn to_lines(&self) -> String {
        use std::fmt::Write;

        let mut lines = String::new();
        for book in &self.books {
            let (title, author, isbn) = (book.title(), book.author(), book.isbn());
            let format = book.format().as_str();
            writeln!(lines, "{title}|{author}|{isbn}|{format}")
                .expect("writing to a String never fails");
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_shelf() -> Shelf {
        let mut shelf = Shelf::new();
        shelf.add(
            "Dune|Frank Herbert|9780441172719|paperback"
                .parse()
                .unwrap(),
        );
        shelf.add(
            "Children of Dune|Frank Herbert|9780441104024|hardcover"
                .parse()
                .unwrap(),
        );
        shelf
    }

    #[test]
    fn search_filters_and_sorts() {
        let shelf = sample_shelf();
        let options = SearchOptions {
            author: Some("Frank Herbert".to_owned()),
            order: SortOrder::TitleDescending,
            ..SearchOptions::default()
        };
        let titles: Vec<&str> = shelf
            .search(&options)
            .into_iter()
            .map(Book::title)
            .collect();
        assert_eq!(titles, ["Dune", "Children of Dune"]);
    }

    #[test]
    #[should_panic(expected = "already on the shelf")]
    fn add_rejects_duplicates() {
        let mut shelf = sample_shelf();
        shelf.add("Dune|Frank Herbert|9780441172719|ebook".parse().unwrap());
    }

    #[test]
    fn take_book_removes_it() {
        let mut shelf = sample_shelf();
        let isbn = Isbn::try_new(9_780_441_172_719).unwrap();
        assert!(shelf.take_book(isbn).is_some());
        assert_eq!(shelf.len(), 1);
    }
}
