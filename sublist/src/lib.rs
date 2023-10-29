#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}


pub fn compare_sublist<T: PartialEq>(first: &[T], second: &[T]) -> bool {

    for window in second.windows(first.len()) {
        if window == first {
            return true;
        }
    }
    
    false
}

pub fn compare_superlist<T: PartialEq>(first: &[T], second: &[T]) -> bool {

    for window in first.windows(second.len()) {
        if window == second {
            return true;
        }
    }
    
    false
}

pub fn compare_equal<T: PartialEq>(first: &[T], second: &[T]) -> bool {

    let not_equal = first.iter().zip(second.iter())
        .any(|(e1, e2)| e1 != e2);

    !not_equal
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    
    let mut res = false;

    if first_list.len() == 0 && second_list.len() == 0 {
        return Comparison::Equal;
    } else if first_list.len() == 0 {
        return Comparison::Sublist;
    } else if second_list.len() == 0 {
        return Comparison::Superlist;
    }
    
    if first_list.len() > second_list.len() {
        res = compare_superlist(first_list, second_list);
        return if res { Comparison::Superlist } else { Comparison::Unequal }
    } else if first_list.len() == second_list.len() {
        res = compare_equal(first_list, second_list);
        return if res { Comparison::Equal } else { Comparison::Unequal }
    } else {
        res = compare_sublist(first_list, second_list);
        return if res { Comparison::Sublist } else { Comparison::Unequal }
    }
}
