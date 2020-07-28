mod Mth;
mod end_gen;
mod simplex_noise;
mod lcg;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
) -> Result<(), JsValue> {
    // The real workhorse of this algorithm, generating pixel data
    let mut data = get_picture(width, height);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn get_picture(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::new();
    for x in 0..width {
        for y in 0..height {
            data.push((x / 4) as u8);
            data.push((y / 2) as u8);
            data.push(x as u8);
            data.push(255);
        }
    }
    data
}

