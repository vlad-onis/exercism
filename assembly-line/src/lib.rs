pub fn production_rate_per_hour(speed: u8) -> f64 {
    
    let units_per_hour = 221.0;
    let speed: f64 = speed.into();
    let result: f64 = 
    
    if speed >= 1.0 && speed <= 4.0 {
        (units_per_hour * speed).into()
    } else if speed >= 5.0 && speed <= 8.0 {
        (units_per_hour * speed as f64 * 0.9).into()
    } else if speed >= 9.0 && speed <= 10.0 {
        (units_per_hour * speed as f64* 0.77) as f64
    } else {
        0.00 as f64
    };

    result
    
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
