use std::time::Duration;
use std::thread;
use std::rc::Rc;
use image::GrayImage;
use unicorn_hat_hd::UnicornHatHd;

const CHAR_SIZE: u32 = 8;
const NB_LINES: u32 = 2;
const FONTMAP_NB_COLS: u32 = 16;
const FONTMAP_NB_LINES: u32 = 8;
const SCREEN_W: u32 = 16;
const SCREEN_H: u32 = 16;

fn main() {

    let mut board = UnicornBoard::new();

    board.set_text(0, "13".into());
    board.set_text(1, "37".into());
    board.display();
    

    loop {
        
        thread::sleep(Duration::from_secs(1));
    }
}


struct UnicornBoard {

    hat_hd: UnicornHatHd,
    lines: Vec<BoardLine>

}

impl UnicornBoard {

    fn new() -> UnicornBoard {

        let font_map = Rc::new(UnicornBoard::load_fontmap());
        let lines = (0..NB_LINES).map(|i| BoardLine::new(font_map.clone(), i)).collect();

        UnicornBoard {
            hat_hd: UnicornHatHd::default(),
            lines
        }
    }

    fn load_fontmap() -> Vec<GrayImage> {

        let mut dyn_image = image::open("font.png").unwrap();

        let mut font_map = Vec::new();
        for j in 0..FONTMAP_NB_LINES {
            for i in 0..FONTMAP_NB_COLS {
                let char_map = dyn_image.crop(i * CHAR_SIZE, j * CHAR_SIZE, CHAR_SIZE, CHAR_SIZE).to_luma();
                font_map.push(char_map);
            }
        }

        font_map
    }

    fn set_text(&mut self, line_index: usize, text: String) {
        self.lines[line_index].set_text(text);
    }

    fn display(&mut self) {

        for line in self.lines.iter() {
            line.display(&mut self.hat_hd);
        }

        self.hat_hd.display().unwrap();
    }

}

struct BoardLine {

    text: String,
    y: u32,
    scroll: bool,
    text_offset: u32,
    font_map: Rc<Vec<GrayImage>>
}

impl BoardLine {

    fn new(font_map: Rc<Vec<GrayImage>>, line_index: u32) -> BoardLine {

        let y = line_index * CHAR_SIZE;

        BoardLine { 
            text: "".to_owned(),
            y,
            scroll: false,
            text_offset: 0,
            font_map: font_map
        }
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }

    fn display(&self, hat_hd: &mut UnicornHatHd) {

        for (i, c) in self.text.chars().enumerate() {
            let x = (i as u32) * CHAR_SIZE;
            self.display_char(hat_hd, c, x, self.y);
        }

    }

    fn display_char(&self, hat_hd: &mut UnicornHatHd, c: char, x: u32, y: u32) {

        let char_map = {
            let index = c as usize;
            if index <= 95 { &self.font_map[index - 1] }
            else { &self.font_map[index] }
        };

        for dx in 0..CHAR_SIZE {
            for dy in 0..CHAR_SIZE {
                
                let active = char_map.get_pixel(dx, dy)[0] > 0;

                let color = if active { [50, 50, 50] } else { [0, 0, 0] };

                let (xp, yp) = ((x + dx) as usize, (y + dy) as usize);
                hat_hd.set_pixel(xp, yp, color.into());

            }
        }

    }

}