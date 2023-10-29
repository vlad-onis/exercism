use enum_iterator::{all, Sequence};
use int_enum::IntEnum;


#[repr(u8)]
#[derive(Debug, PartialEq, Eq, IntEnum, Clone, Copy, Sequence, PartialOrd, Ord)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey= 8,
    Orange= 3,
    Red= 2,
    Violet= 7,
    White= 9,
    Yellow= 4,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value() as u32
}

pub fn value_to_color_string(value: u32) -> String {
    match value {
        0 => { String::from("Black") },
        1 => { String::from("Brown") },
        2 => { String::from("Red") },
        3 => { String::from("Orange") },
        4 => { String::from("Yellow") },
        5 => { String::from("Green") },
        6 => { String::from("Blue") },
        7 => { String::from("Violet") },
        8 => { String::from("Grey") },
        9 => { String::from("White") },
        _ => { String::from("value out of range") },
    }   
}

pub fn colors() -> Vec<ResistorColor> {
    let mut res = all::<ResistorColor>().collect::<Vec<ResistorColor>>();
    res.sort();
    res
}
