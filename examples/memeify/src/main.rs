use ab_glyph::{FontRef, PxScale};
use image::{EncodableLayout, Rgba};
use imageproc::drawing::draw_text_mut;
use shared::{run_function, FunctionType};

mod font {
    include!(concat!(env!("OUT_DIR"), "/font.rs"));
}
mod img {
    include!(concat!(env!("OUT_DIR"), "/img.rs"));
}

// Reads std in as input, retrieves the fibonacci sequence and returns the last number of the
// fibonacci sequence of the provided size
fn main() {
    let func_type = if cfg!(feature = "docker") {
        FunctionType::Docker
    } else {
        FunctionType::Wasm
    };

    run_function(memeify, func_type);
}

fn memeify(text: String) -> String {
    // Load the image from a &[u8] representation (replace `image_bytes` with your actual image data)

    let parts: Vec<&str> = text.split('|').map(|s| s.trim()).collect();

    let (top_text, bottom_text) = if let [first, second] = parts.as_slice() {
        (*first, *second)
    } else {
        ("You know what", "I'm somewhat of a master student as well")
    };

    // let image_bytes: &[u8] = include_bytes!("../neko-punk.png");
    // println!("{:?}", image_bytes);
    let mut image = image::load_from_memory(img::IMG_BYTES).unwrap().to_rgba8();
    let (_w, h) = image.dimensions();

    // let width = w as i32;
    let height = h as i32;

    let text_height = (height / 10) as u32;
    let text_padding = height / 20;

    let font = FontRef::try_from_slice(font::FONT_BYTES).unwrap();

    let scale = PxScale { x: 40.0, y: 40.0 };

    draw_text_mut(
        &mut image,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        text_padding,
        text_padding,
        scale,
        &font,
        top_text,
    );
    draw_text_mut(
        &mut image,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        text_padding,
        height - (text_height as i32) - text_padding,
        scale,
        &font,
        bottom_text,
    );

    format!("{:?}", image.as_bytes().len())
}
