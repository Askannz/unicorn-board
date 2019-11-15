mod unicorn_board;

use std::thread;
use std::time::Duration;
use unicorn_board::{UnicornBoard, Scroll};

fn main() {

    let mut board = UnicornBoard::new();

    let line_1 = board.new_line(0, "ILoveRust  ").with_color(127, 63, 0).with_scroll(Scroll::Left(16.0)).build();
    let line_2 = board.new_line(8, "####").with_color(127, 127, 127).with_scroll(Scroll::Left(8.0)).build();
    board.add_line(line_1);
    board.add_line(line_2);

    loop {
        board.display();
        thread::sleep(Duration::from_millis(10));
    }
}

