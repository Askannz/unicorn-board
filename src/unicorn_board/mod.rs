mod font;

use std::time::{Instant, Duration};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use image::{GrayImage, RgbImage, Rgb};
use unicorn_hat_hd::{UnicornHatHd, Rotate};

const SCREEN_W: u32 = 16;
const SCREEN_H: u32 = 16;

pub type Font = font::Font;

#[derive(Clone, Copy)]
pub enum Scroll {
    Off,
    Left { speed: f32, spacing: u32 },
    Right { speed: f32, spacing: u32 },
    LeftAuto { speed: f32, spacing: u32 },
    RightAuto { speed: f32, spacing: u32 },
}

pub struct UnicornBoard {
    thread_control: Option<(Arc<AtomicBool>, JoinHandle<()>)>
}

impl UnicornBoard {

    pub fn new() -> UnicornBoard {
        UnicornBoard { thread_control: None }
    }

    pub fn activate(&mut self, line_configs_list: &[Line]) {

        let running = Arc::new(AtomicBool::new(true));

        let line_configs_list: Vec<Line> = line_configs_list.into();

        if line_configs_list.is_empty() { return }

        let handle = std::thread::spawn({let running = running.clone(); move || {

            let font_maps = font::load_fontmaps();

            let mut hat_hd = UnicornHatHd::default();
            hat_hd.set_rotation(Rotate::Rot180);

            let mut lines = Vec::new();
            let mut y = 0;
            for line_config in line_configs_list.iter() {
                lines.push(BoardLine::new(&font_maps, y, line_config.clone()));
                y += font_maps[&line_config.font][0].height();
            }

            while running.load(Ordering::SeqCst) {

                hat_hd.display().unwrap();

                let next_t = lines.iter_mut().map(|line| line.update(&mut hat_hd) ).min().unwrap();

                std::thread::sleep(next_t - Instant::now());
            }

            hat_hd.clear_pixels();
            hat_hd.display().unwrap();
            
        }});

        self.thread_control = Some((running, handle));
    }
}

impl Drop for UnicornBoard {
    fn drop(&mut self) {
        let thread_control = std::mem::replace(&mut self.thread_control, None);
        if let Some((running, handle)) = thread_control {
            running.store(false, Ordering::SeqCst);
            handle.join().unwrap();
        }
    }
}

#[derive(Clone)]
pub struct Line {
    scroll_mode: Scroll,
    text: String,
    color: (u8, u8, u8),
    font: Font
}

impl Line {

    pub fn new(text: &str) -> Line {

        Line {
            scroll_mode: Scroll::Off,
            text: text.into(),
            color: (255, 255, 255),
            font: Font::Big8x8
        }

    }

    pub fn with_color(&self, r: u8, g: u8, b: u8) -> Line {
        let mut new_line = (*self).clone();
        new_line.color = (r, g, b);
        new_line
    }

    pub fn with_scroll(&self, scroll_mode: Scroll) -> Line {
        let mut new_line = self.clone();
        new_line.scroll_mode = scroll_mode;
        new_line
    }

    pub fn with_font(&self, font: Font) -> Line {
        let mut new_line = self.clone();
        new_line.font = font;
        new_line
    }

}

pub struct BoardLine {

    y: u32,
    scroll_speed: f32,
    x_offset: u32,
    prev_instant: Instant,
    pixmap: RgbImage,
}

impl BoardLine {

    fn new(font_maps: &HashMap<Font, Vec<GrayImage>>, y: u32, line_config: Line) -> BoardLine {

        let Line { scroll_mode, text, color, font } = line_config;

        let n = (SCREEN_W / font_maps[&font][0].width()) as usize;

        let scroll_speed = match scroll_mode {
            Scroll::Off => 0.0,
            Scroll::Left { speed, spacing: _ } => speed,
            Scroll::Right { speed, spacing: _ } => -speed,
            Scroll::LeftAuto { speed, spacing: _ } => if text.len() > n { speed } else { 0.0 },
            Scroll::RightAuto { speed, spacing: _ } => if text.len() > n { -speed } else { 0.0 },
        };

        let text = match scroll_mode {

            Scroll::Left { speed: _, spacing } | Scroll::LeftAuto { speed: _, spacing } => {
                text + &String::from(" ").repeat(spacing as usize)
            },

            Scroll::Right { speed: _, spacing } | Scroll::RightAuto { speed: _, spacing } => {
                String::from(" ").repeat(spacing as usize) + &text
            },

            _ => text.clone()

        };

        BoardLine { 
            y,
            scroll_speed,
            x_offset: 0,
            prev_instant: Instant::now(),
            pixmap: BoardLine::make_pixmap(&font_maps[&font], &text, color)
        }
    }

    fn make_pixmap(font_map: &Vec<GrayImage>, text: &String, color: (u8, u8, u8)) -> RgbImage {

        let (char_w, char_h) = font_map[0].dimensions();
        let n = (SCREEN_W / char_w)  as usize;

        let padded_text: String = {
            if text.len() < n { format!("{: <1$}", text, n) }
            else { text.clone() }
        };

        let pixmap_w = padded_text.len() as u32 * char_w;
        let pixmap_h = char_h;

        let mut pixmap = RgbImage::new(pixmap_w, pixmap_h);

        for (i, c) in padded_text.chars().enumerate() {
            for dx in 0..char_w {
                for dy in 0..char_h {

                    let x = (i as u32) * char_w + dx;
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

    fn update(&mut self, hat_hd: &mut UnicornHatHd) -> Instant {
        let next_t = self.update_scroll();
        self.display(hat_hd);
        next_t
    }

    fn display(&self, hat_hd: &mut UnicornHatHd) {

        let (pixmap_w, pixmap_h) = self.pixmap.dimensions();

        for dx in 0..SCREEN_W {
            for dy in 0..pixmap_h {

                let x_pixmap = (self.x_offset + dx) % pixmap_w;
                let y_pixmap = dy;

                let x_screen = dx;
                let y_screen = self.y + dy;

                let Rgb(color) = self.pixmap.get_pixel(x_pixmap, y_pixmap);
                hat_hd.set_pixel(x_screen as usize, y_screen as usize, (*color).into());
            }

        }
    }

    fn update_scroll(&mut self) -> Instant {

        let inc = if self.scroll_speed.is_sign_positive() { 1 } else { -1 };

        let dt = (1000.0 / self.scroll_speed.abs()) as u128;
        let now = Instant::now();
        if now.duration_since(self.prev_instant).as_millis() > dt {
            let x_offset_inc: i32 = self.x_offset as i32 + inc;
            self.x_offset = x_offset_inc.rem_euclid(self.pixmap.width() as i32) as u32;
            self.prev_instant = now;
        }

        Instant::now() + Duration::from_millis(dt as u64)
    }

} 
