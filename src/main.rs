mod ray;

use glam::{Vec3A, Mat4};
use ray::Ray;
use std::fs::File;
use std::io::Write;

fn main()
{
    // Img
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMG_WIDTH: i32 = 400;
    const IMG_HEIGHT: i32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0f32;

    let origin = Vec3A::ZERO;
    let horizontal = Vec3A::new(viewport_width, 0f32, 0f32);
    let vertical = Vec3A::new(0f32, viewport_height, 0f32);
    let lower_left_corner = origin - horizontal / 2f32
        - vertical / 2f32
        - Vec3A::new(0f32, 0f32, focal_length);

    let mut out = format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT);

    // This is how reverse loops are done in Rust.
    for j in (0..IMG_HEIGHT).rev() {
        for i in 0..IMG_WIDTH {
            let bg_color = Vec3A::new(0f32, 150f32, 255f32);

            print!("{}\n", bg_color);
            let data = format!("{} {} {}\n", bg_color.x, bg_color.y, bg_color.z);
            out += &data;
        }
    }

    let mut file = File::create("out.ppm").expect("Cannot create file.");
    file.write_all(out.as_bytes()).expect("Could not write data.");
}
