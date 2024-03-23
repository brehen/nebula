use std::{fs::File, io::Write, path::Path};

fn main() {
    // Read the font file as bytes
    let font_path = Path::new("./arial.ttf");
    let font_bytes = std::fs::read(font_path).expect("Failed to read font file");

    let base_img_path = Path::new("./marius.jpeg");
    let img_bytes = std::fs::read(base_img_path).expect("Failed to read img file");

    // Write the font bytes to a Rust file
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let font_dest_path = Path::new(&out_dir).join("font.rs");
    let mut font_dest_file = File::create(font_dest_path).expect("Failed to create font.rs");

    let img_dest_path = Path::new(&out_dir).join("img.rs");
    let mut img_dest_file = File::create(img_dest_path).expect("Failed to create img.rs");

    // Convert the byte vector to a comma-separated string
    let byte_str = font_bytes
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    let img_byte_str = img_bytes
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    font_dest_file
        .write_all(format!("pub const FONT_BYTES: &[u8] = &[{}];", byte_str).as_bytes())
        .expect("Failed to write font bytes to font.rs");
    img_dest_file
        .write_all(format!("pub const IMG_BYTES: &[u8] = &[{}];", img_byte_str).as_bytes())
        .expect("Failed to write font bytes to img.rs");

    println!("cargo:rerun-if-changed=build.rs");
}
