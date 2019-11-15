mod unicorn_board;

use std::thread;
use std::time::Duration;
use unicorn_board::{UnicornBoard, Scroll, Line};

fn main() {

    let mut board = UnicornBoard::new();

    let line_1 = Line::new("ILoveRust  ", 0).with_color(127, 63, 0).with_scroll(Scroll::Left(16.0));
    let line_2 = Line::new("####", 8).with_color(127, 127, 127).with_scroll(Scroll::Left(8.0));

    board.add_line(line_1);
    board.add_line(line_2);

    loop {
        board.display();
        thread::sleep(Duration::from_millis(10));
    }
}

