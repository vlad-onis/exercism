pub fn is_armstrong_number(num: u32) -> bool {
    let mut num_clone = num.clone();
    let mut sum: u64 = 0;
    let digits = ((num + 1) as f32).log10().ceil() as u32;
    while num_clone != 0 {
        sum += u32::pow(num_clone % 10, digits) as u64;
        num_clone /= 10;
    }

    sum == num as u64
}
