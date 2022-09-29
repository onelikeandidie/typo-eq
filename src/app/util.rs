use crossterm::terminal;

pub fn term_center() -> (u16, u16) {
    let (width, height) = terminal::size()
        .expect("Could not get terminal window size");
    let x = width / 2;
    let y = height / 2;
    (x, y)
}