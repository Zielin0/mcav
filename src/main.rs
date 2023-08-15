use std::{env, path::Path, process};

use colored::*;
use image::{GenericImageView, Rgba};

fn fail(message: &str) {
    println!("ERROR: {message}");
    process::exit(1)
}

fn print_head(pixels: &Vec<Rgba<u8>>) {
    for y in 0..8 {
        for x in 0..8 {
            let index = y * 8 + x;
            if let Some(color) = pixels.get(index) {
                print!("{}", "  ".on_truecolor(color[0], color[1], color[2]))
            }
        }
        print!("\n")
    }
}

fn main() {
    match env::args().nth(1) {
        Some(skin_path) => {
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

            print_head(&pixels)
        }
        None => fail("Missing a path to skin.png"),
    }
}
