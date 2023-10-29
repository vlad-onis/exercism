use time::{PrimitiveDateTime as DateTime, Date, Time, OffsetDateTime};
use time::{Duration};


// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    
    let giga_second = Duration::new(1000000000, 0);
    start + giga_second
}
