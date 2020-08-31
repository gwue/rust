pub fn verse(n: u32) -> String {
    let line_1 = format!("{} bottles of beer on the wall, {} bottles of beer.", n, n);
    let line_2 = match n {
        1 => "Take it down and pass it around, no more bottles of beer on the wall.".to_string(),
        _ => format!(
            "Take one down and pass it around, {} bottle of beer on the wall.",
            n - 1
        ),
    };

    format!("{}\n{}", line_1, line_2)
}

pub fn end_verse(n: u32) -> String {
    format!(
        "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, {} bottles of beer on the wall.",
        n
    )
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    for i in start..end {
        let line = match i {
            0 => end_verse(start),
            _ => verse(i),
        };
        song.push_str(&line.as_str());
    }

    song
}
