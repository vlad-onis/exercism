pub fn square_of_sum(n: u32) -> u32 {
    // Unwrapping here because the return type is u32
    // There's no nice way to handle the None case
    // The problem assumes basically that no overflow will occur
    let sum = n.checked_mul(n + 1).unwrap() / 2;
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..n + 1).into_iter().map(|elem| elem * elem).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
