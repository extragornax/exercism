use std::collections::HashMap;

pub fn is_letter(letter: char) -> bool {
    if letter.is_ascii_alphabetic() {
        return true;
    }
    return false;
}

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    if sentence.len() < 26 {
        return false;
    }

    let mut items = HashMap::new();

    for letter in sentence.chars() {
        if is_letter(letter) {
            items.insert(letter, 1);
        }
    }

    if items.len() < 26 {
        return false;
    }

    return true;
}
