/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.trim();
    if code.len() <= 1 {
        return false;
    }

    let v: Vec<bool> = code
        .chars()
        .map(|c| c.is_whitespace() || c.is_numeric())
        .collect();
    println!("{v:?}\n{code}");
    if v.contains(&false) {
        return false;
    }

    let mut s = code.to_string();
    s.retain(|c| !c.is_whitespace());

    let mut digits: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap_or(0)).collect();
    digits.reverse();

    let sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(index, element)| {
            if index % 2 != 0 {
                let element = element * 2;
                if element >= 10 {
                    element - 9
                } else {
                    element
                }
            } else {
                element.to_owned()
            }
        })
        .sum();

    sum % 10 == 0
}
