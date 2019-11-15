mod unicorn_board;

use std::env;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
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

    let line = Line::new(&text, 4).with_color(127, 63, 0).with_scroll(Scroll::LeftAuto { speed: 16.0, spacing: 5 });
    board.set_lines(&[line]);

    let running = Arc::new(AtomicBool::new(true));

    ctrlc::set_handler({ let running = running.clone(); move || {
        running.store(false, Ordering::SeqCst);
    }}).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        board.display();
        thread::sleep(Duration::from_millis(10));
    }
}

