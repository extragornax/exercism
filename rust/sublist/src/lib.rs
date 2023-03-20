use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (x, y) if x > y => {
            if _first_list.windows(y).any(|v| v == _second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (x, y) if x < y => {
            if _second_list.windows(x).any(|v| v == _first_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (_, _) => {
            if _first_list == _second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}
