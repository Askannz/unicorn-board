use std::time::{Duration, Instant};
use std::thread;
use std::rc::Rc;
use image::{GrayImage, RgbImage, Rgb};
use unicorn_hat_hd::{UnicornHatHd, Rotate};

const CHAR_W: u32 = 4;
const CHAR_H: u32 = 5;
const FONTMAP_X0: u32 = 0;
const FONTMAP_Y0: u32 = 1;
const FONTMAP_STRIDE_X: u32 = 5;
const FONTMAP_STRIDE_Y: u32 = 7;
const FONTMAP_NB_COLS: u32 = 32;
const FONTMAP_NB_LINES: u32 = 4;

const SCREEN_W: u32 = 16;
const SCREEN_H: u32 = 16;

const NB_LINES: u32 = 3;

const MAX_CHARS_PER_LINE: u32 = SCREEN_W / CHAR_W;

fn main() {

    let mut board = UnicornBoard::new();

    board.set_text(0, "OREWA GUNDAMU !!!!".into(), (255, 0, 0), Scroll::On(8.0));
    board.set_text(1, "------------------".into(), (0, 255, 0), Scroll::Off);
    board.set_text(2, "123456789".into(), (0, 0, 255), Scroll::Off);
    board.display();

    loop {
        board.update_scroll();
        board.display();
        thread::sleep(Duration::from_millis(10));
    }
}

enum Scroll {
    Off,
    On(f32)
}

struct UnicornBoard {

    hat_hd: UnicornHatHd,
    lines: Vec<BoardLine>

}

impl UnicornBoard {

    fn new() -> UnicornBoard {

        let font_map = Rc::new(UnicornBoard::load_fontmap());
        let lines = (0..NB_LINES).map(|i| BoardLine::new(font_map.clone(), i)).collect();

        let mut hat_hd = UnicornHatHd::default();
        hat_hd.set_rotation(Rotate::Rot180);

        UnicornBoard { hat_hd, lines }
    }

    fn load_fontmap() -> Vec<GrayImage> {

        let mut dyn_image = image::open("font.png").unwrap();

        let mut font_map = Vec::new();
        for j in 0..FONTMAP_NB_LINES {
            for i in 0..FONTMAP_NB_COLS {
                let char_map = dyn_image.crop(
                    FONTMAP_X0 + i * FONTMAP_STRIDE_X,
                    FONTMAP_Y0 + j * FONTMAP_STRIDE_Y,
                    CHAR_W,
                    CHAR_H).to_luma();
                font_map.push(char_map);
            }
        }

        font_map
    }

    fn set_text(&mut self, line_index: usize, text: String, color: (u8, u8, u8), scroll_mode: Scroll) {
        self.lines[line_index].set_text(text, color, scroll_mode);
    }

    fn display(&mut self) {

        for line in self.lines.iter() {
            line.display(&mut self.hat_hd);
        }

        self.hat_hd.display().unwrap();
    }

    fn update_scroll(&mut self) {
        for line in self.lines.iter_mut() {
            line.update_scroll();
        }
    }

}

struct BoardLine {

    y: u32,
    scroll_mode: Scroll,
    x_offset: u32,
    prev_instant: Instant,
    pixmap: RgbImage,
    font_map: Rc<Vec<GrayImage>>
}

impl BoardLine {

    fn new(font_map: Rc<Vec<GrayImage>>, line_index: u32) -> BoardLine {

        let y = line_index * CHAR_H;

        BoardLine { 
            y,
            scroll_mode: Scroll::Off,
            x_offset: 0,
            prev_instant: Instant::now(),
            pixmap: BoardLine::make_pixmap(&font_map, "".into(), (0, 0, 0)),
            font_map: font_map
        }
    }

    fn make_pixmap(font_map: &Vec<GrayImage>, text: String, color: (u8, u8, u8)) -> RgbImage {

        let n = MAX_CHARS_PER_LINE as usize;

        let padded_text = {
            if text.len() < n { format!("{: <1$}", text, n) }
            else { text }
        };

        let pixmap_w = padded_text.len() as u32 * CHAR_W;
        let pixmap_h = CHAR_H;

        let mut pixmap = RgbImage::new(pixmap_w, pixmap_h);

        for (i, c) in padded_text.chars().enumerate() {
            for dx in 0..CHAR_W {
                for dy in 0..CHAR_H {

                    let x = (i as u32) * CHAR_W + dx;
                    let y = dy;

                    let active = font_map[c as usize].get_pixel(dx, dy)[0] > 0;
                    let color = if active { color } else { (0, 0, 0) };

                    let (r, g, b) = color;
                    *pixmap.get_pixel_mut(x, y) = Rgb([r, g, b]);
                }
            }
        }

        pixmap
    }

    fn set_text(&mut self, text: String, color: (u8, u8, u8), scroll_mode: Scroll) {

        self.pixmap = BoardLine::make_pixmap(&self.font_map, text, color);
        self.scroll_mode = scroll_mode;
        self.x_offset = 0;
    }

    fn display(&self, hat_hd: &mut UnicornHatHd) {

        let x_offset = match self.scroll_mode {
            Scroll::Off => 0,
            Scroll::On(_) => self.x_offset
        };

        let pixmap_w = self.pixmap.width();

        for dx in 0..SCREEN_W {
            for dy in 0..CHAR_H {

                let x_pixmap = (x_offset + dx) % pixmap_w;
                let y_pixmap = dy;

                let x_screen = dx;
                let y_screen = self.y + dy;

                let Rgb(color) = self.pixmap.get_pixel(x_pixmap, y_pixmap);
                hat_hd.set_pixel(x_screen as usize, y_screen as usize, (*color).into());
            }

        }
    }

    fn update_scroll(&mut self) {

        let dt = match self.scroll_mode {
            Scroll::Off => 0,
            Scroll::On(speed) => (1000.0 / speed) as u128
        };

        let now = Instant::now();
        if now.duration_since(self.prev_instant).as_millis() > dt {
            self.x_offset = (self.x_offset + 1) % (self.pixmap.width() as u32);
            self.prev_instant = now;
        }
    }

}