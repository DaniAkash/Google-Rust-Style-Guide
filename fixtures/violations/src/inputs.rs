//! Function inputs and outputs.

/// Accepts a reference to a box instead of the boxed value.
#[must_use]
pub fn boxed_len(value: &Box<String>) -> usize {
    //~^ clippy::borrowed_box
    value.len()
}

/// Accepts a reference to an `Option` instead of an `Option` of a reference.
#[must_use]
pub fn name_len(name: &Option<String>) -> usize {
    //~^ clippy::ref_option
    name.as_ref().map_or(0, String::len)
}

/// Two bool flags whose meaning is invisible at the call site.
pub fn render(bold: bool, italic: bool) {
    //~^ clippy::fn_params_excessive_bools
    let _ = (bold, italic);
}

/// Two bool fields instead of an enum.
#[non_exhaustive]
pub struct Style {
    //~^ clippy::struct_excessive_bools
    /// Bold text.
    pub bold: bool,
    /// Italic text.
    pub italic: bool,
}
