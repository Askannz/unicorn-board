mod unicorn_board;

use std::env;
use unicorn_board::{UnicornBoard, Scroll, Line, Font};

fn main() {

    let texts_list: Vec<String> = env::args().skip(1).collect();

    let fonts_list: Vec<Font> = vec![Font::Small5x5, Font::Big8x8];

    let lines_list: Vec<Line> = texts_list.iter().enumerate().map(|(i, text)| {
        let font = fonts_list[i % fonts_list.len()];
        Line::new(&text)
        .with_color(127, 63, 0)
        .with_scroll(Scroll::LeftAuto { speed: 16.0, wrap_gap: 5 })
        .with_font(font)
    }).collect();

    let mut board = UnicornBoard::new();
    board.activate(&lines_list);

    println!("Press ENTER to continue");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

