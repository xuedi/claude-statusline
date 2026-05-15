use std::io::{self, Read};

fn main() {
    let mut raw = String::new();
    let _ = io::stdin().read_to_string(&mut raw);
    print!("{}", claude_statusline::render(&raw));
}
