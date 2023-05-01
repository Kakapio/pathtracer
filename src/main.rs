mod Ray;

use glam::{Vec3A, Mat4};
use std::fs::File;
use std::io::Write;

fn main() {
    const IMG_WIDTH: i32 = 512;
    const IMG_HEIGHT: i32 = 512;

    let mut out = format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);

    // This is how reverse loops are done in Rust.
    for j in (0..IMG_HEIGHT).rev() {
        for i in 0..IMG_HEIGHT {
            let r = i as f64 / (IMG_WIDTH - 1) as f64;
            let g = j as f64 / (IMG_HEIGHT - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            let data = format!("{} {} {}\n", ir, ig, ib);
            out += &data;
        }
    }

    let mut file = File::create("out.ppm").expect("Cannot create file.");
    file.write_all(out.as_bytes()).expect("Could not write data.");
}
