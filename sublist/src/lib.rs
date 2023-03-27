#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list, _second_list) {
        (first, second) if first == second => Comparison::Equal,
        (first, second) if first.is_empty() => Comparison::Sublist,
        (first, second) if second.is_empty() => Comparison::Superlist,
        (first, second) if first.len() > second.len() => {
            if first.windows(second.len()).any(|w| w == second) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (first, second) if first.len() < second.len() => {
            if second.windows(first.len()).any(|w| w == first) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        _ => Comparison::Unequal,
       
    }
}
