mod unicorn_board;

use std::env;
use std::sync::mpsc::channel;
use unicorn_board::{UnicornBoard, Scroll, Line, Font};

fn main() {

    let texts_list: Vec<String> = env::args().skip(1).take(2).collect();

    let lines_list: Vec<Line> = texts_list.iter().enumerate().map(|(i, text)| {

        let font = [Font::Big8x8, Font::Small5x5][i];
        let (r, g, b) = [(127, 63, 0), (100, 100, 100)][i];
        let speed = [20.0, 10.0][i];

        Line::new(&text)
                .with_color(r, g, b)
                .with_scroll(Scroll::LeftAuto { speed, wrap_gap: 1 })
                .with_font(font)

    }).collect();

    let mut board = UnicornBoard::new();
    board.activate(&lines_list);

    let (sender, receiver) = channel();

    ctrlc::set_handler({let sender = sender.clone(); move || {
        println!("Received SIGTERM");
        sender.send(()).unwrap();
    }}).expect("Error setting SIGTERM handler");

    receiver.recv().unwrap();
    println!("Exiting. Goodbye !")
}

