use std::collections::HashSet;


pub fn factors(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }

    let mut candidates = 2..;

    let mut res = Vec::new();

    let mut number_copy = n.clone();


    while number_copy > 1 {
        let factor = candidates.next().unwrap();

        while number_copy % factor == 0 {
            number_copy /= factor;
            res.push(factor);
        }
    }

    res
}
