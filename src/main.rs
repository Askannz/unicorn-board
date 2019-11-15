mod unicorn_board;

use std::env;
use std::thread;
use std::time::Duration;
use unicorn_board::{UnicornBoard, Scroll, Line};

fn main() {

    let text = match env::args().skip(1).next() {
        Some(text) => text,
        None => {
            println!("No text specified !");
            std::process::exit(1)
        }
    };

    let mut board = UnicornBoard::new();

    let line = Line::new(&text, 4).with_color(127, 63, 0).with_scroll(Scroll::Left(16.0));
    board.add_line(line);

    loop {
        board.display();
        thread::sleep(Duration::from_millis(10));
    }
}

