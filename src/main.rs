use std::{env, process};

use base64::{engine::general_purpose, Engine};
use colored::*;
use image::{DynamicImage, GenericImageView, Rgba};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct UuidResponse {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Property {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SkinResponse {
    id: String,
    name: String,
    properties: Vec<Property>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SkinTexture {
    metadata: Value,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Textures {
    #[serde(rename = "SKIN")]
    skin: SkinTexture,
}

#[derive(Serialize, Deserialize, Debug)]
struct SkinProfile {
    timestamp: u64,
    #[serde(rename = "profileId")]
    profile_id: String,
    #[serde(rename = "profileName")]
    profile_name: String,
    textures: Textures,
}

async fn get_uuid(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(format!(
        "https://api.mojang.com/users/profiles/minecraft/{}",
        name
    ))
    .await?
    .json::<UuidResponse>()
    .await?;

    Ok(response.id)
}

async fn get_skin_base64(uuid: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid
    ))
    .await?
    .json::<SkinResponse>()
    .await?;

    Ok(response.properties[0].value.clone())
}

async fn get_skin(skin_url: String) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let response = reqwest::get(skin_url).await?.bytes().await?;
    let skin = image::load_from_memory(&response)?;

    Ok(skin)
}

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

#[tokio::main]
async fn main() {
    match env::args().nth(1) {
        Some(username) => {
            let mut scale = 1;
            if let Some(scale_arg) = env::args().nth(2) {
                scale = scale_arg.parse::<usize>().unwrap();
            }

            let uuid = get_uuid(&username).await.unwrap_or_else(|_| {
                fail(format!("Player '{}' doesn't exist.", username).as_str());
                Default::default()
            });
            let skin_data_base64 = get_skin_base64(uuid.as_str()).await.unwrap();

            let skin_bytes = general_purpose::STANDARD.decode(skin_data_base64).unwrap();
            let skin_json = String::from_utf8_lossy(&skin_bytes).to_string();
            let parsed_skin_json: SkinProfile = serde_json::from_str(&skin_json).unwrap();

            let skin = get_skin(parsed_skin_json.textures.skin.url).await.unwrap();

            if skin.dimensions() != (64, 64) {
                fail("Wrong skin size. Should never happen.")
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
        None => fail("Missing a username"),
    }
}
