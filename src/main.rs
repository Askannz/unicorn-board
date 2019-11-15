mod unicorn_board;

use std::env;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use unicorn_board::{UnicornBoard, Scroll, Line};

fn main() {

    let texts_list: Vec<String> = env::args().skip(1).collect();
    let mut board = UnicornBoard::new();

    let lines_list: Vec<Line> = texts_list.iter().map(|text| {
        Line::new(&text).with_color(127, 63, 0).with_scroll(Scroll::LeftAuto { speed: 16.0, spacing: 5 })
    }).collect();

    board.set_lines(&lines_list);

    let running = Arc::new(AtomicBool::new(true));

    ctrlc::set_handler({ let running = running.clone(); move || {
        running.store(false, Ordering::SeqCst);
    }}).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        board.display();
        thread::sleep(Duration::from_millis(10));
    }
}

