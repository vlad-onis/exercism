// 99 bottles of beer on the wall, 99 bottles of beer.
// Take one down and pass it around, 98 bottles of beer on the wall.

pub fn verse(n: u32) -> String {
    match n {
        0 => {
            format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
        }
        1 => {
            format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
        }
        2 => {
            format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n")
        }
        elem @ 3.. => {
            format!("{elem} bottles of beer on the wall, {elem} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", elem-1)
        }
        _ => {
            format!("KEKOVICI")
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..start + 1)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
