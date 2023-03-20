use std::ops::Range;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

pub fn is_palindrome(value: u64) -> bool {
    value.to_string() == value.to_string().chars().rev().collect::<String>()
}

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let i = min..max + 1;

    let e = i.into_iter().fold(vec![], |acc, x| {
        let range: Range<u64> = x..max + 1;
        range.into_iter().fold(acc, |mut acc, y| {
            let product = x * y;
            if is_palindrome(product) {
                acc.push(product);
            }
            acc
        })
    });

    let min = e.iter().min();
    let max = e.iter().max();

    match (min, max) {
        (Some(min), Some(max)) => Some((Palindrome(*min), Palindrome(*max))),
        _ => None,
    }
}
