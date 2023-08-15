use std::{env, path::Path, process};

use colored::*;
use image::{GenericImageView, Rgba};

fn fail(message: &str) {
    println!("ERROR: {}", message);
    process::exit(1)
}

fn print_head(pixels: &Vec<Rgba<u8>>, scale: usize) {
    for y in 0..8 * scale {
        for x in 0..8 * scale {
            for _ in 0..scale {
                let index = (y / scale) * 8 + (x / scale);
                if let Some(color) = pixels.get(index) {
                    let space = if scale == 1 { "  " } else { " " };
                    print!("{}", space.on_truecolor(color[0], color[1], color[2]));
                }
            }
        }
        print!("\n");
    }
}

fn main() {
    match env::args().nth(1) {
        Some(skin_path) => {
            let mut scale = 1;
            if let Some(scale_arg) = env::args().nth(2) {
                scale = scale_arg.parse::<usize>().unwrap();
            }

            let is_file = Path::new(skin_path.as_str()).is_file();
            if !is_file {
                fail(format!("File '{}' doesn't exist", skin_path).as_str())
            }

            let skin = image::open(skin_path).unwrap();
            if skin.dimensions() != (64, 64) {
                fail("Wrong skin size. Please input 64x64 version.")
            }

            let mut pixels: Vec<Rgba<u8>> = vec![];
            for (x, y, pixel) in skin.pixels() {
                if y >= 8 && y <= 15 && x >= 8 && x <= 15 {
                    pixels.push(pixel)
                }
            }

            let mut pixel_index = 0;
            for (x, y, pixel) in skin.pixels() {
                if y >= 8 && y <= 15 && x >= 40 && x <= 47 {
                    if pixel[3] != 0 {
                        if let Some(pixel_mod) = pixels.get_mut(pixel_index) {
                            *pixel_mod = pixel;
                        }
                    }
                    pixel_index += 1;
                }
            }

            print_head(&pixels, scale)
        }
        None => fail("Missing a path to skin.png"),
    }
}
