use std::collections::HashMap;
use image::GrayImage;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Font {
    Small5x5,
    Big8x8
}

pub fn load_fontmaps() -> HashMap<Font, Vec<GrayImage>> {

    let make_fontmap = |font_meta: FontMeta| {

        let mut dyn_image = image::open(font_meta.filepath).unwrap();

        let mut font_map = Vec::new();
        for j in 0..font_meta.map_dims.1 {
            for i in 0..font_meta.map_dims.0 {
                let char_map = dyn_image.crop(
                    font_meta.origin.0 + i * font_meta.stride.0,
                    font_meta.origin.1 + j * font_meta.stride.1,
                    font_meta.char_dims.0,
                    font_meta.char_dims.1).to_luma();
                font_map.push(char_map);
            }
        }

        font_map
    };

    get_fonts_meta_info().iter().map(|(font, font_meta)| {
        (*font, make_fontmap(font_meta.clone()))
    }).collect()
}


#[derive(Clone)]
struct FontMeta {
    filepath: &'static str,
    origin: (u32, u32),
    stride: (u32, u32),
    char_dims: (u32, u32),
    map_dims: (u32, u32)
}

fn get_fonts_meta_info() -> HashMap<Font, FontMeta> {

    let font_5x5_meta = FontMeta {
        filepath: "fonts/kongtext.png",
        origin: (0, 0),
        stride: (8, 8),
        char_dims: (8, 8),
        map_dims: (32, 4)
    };

    let font_8x8_meta = FontMeta {
        filepath: "fonts/magero.png",
        origin: (0, 0),
        stride: (5, 5),
        char_dims: (5, 5),
        map_dims: (32, 4)
    };

    [(Font::Small5x5, font_5x5_meta),
    (Font::Big8x8, font_8x8_meta)]
    .iter().cloned().collect()
} 