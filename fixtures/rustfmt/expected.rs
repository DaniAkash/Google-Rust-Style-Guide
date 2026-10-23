use std::collections::HashMap;
use std::fmt::{self, Display};

const MASK: u32 = 0xff00ab;

macro_rules! pair {
    ($left:expr, $right:expr) => {
        ($left, $right)
    };
}

pub fn describe(values: &HashMap<String, u32>, label: &dyn Display) -> fmt::Result {
    let _ = (values, label, MASK, pair!(1, 2));
    Ok(())
}

pub fn summary(first_value: u32, second_value: u32, third_value: u32) -> u32 {
    first_value
        .saturating_add(second_value)
        .saturating_add(third_value)
}
