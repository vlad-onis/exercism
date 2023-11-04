pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    (2 as u64).pow(s.checked_sub(1).unwrap_or(0))
}

pub fn total() -> u64 {
    (1..65).into_iter().map(|s| square(s)).sum()
}
