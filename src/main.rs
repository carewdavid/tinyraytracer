use std::io::prelude::*;
use std::fs::File;
use std::f32;

#[derive(Copy, Clone)]
struct Pixel(f32, f32, f32);


impl Pixel {
    fn as_bytes(self) -> [u8; 3] {
        [
            (255.0 * 0.0_f32.max(1.0_f32.min(self.0))) as u8,
            (255.0 * 0.0_f32.max(1.0_f32.min(self.1))) as u8,
            (255.0 * 0.0_f32.max(1.0_f32.min(self.2))) as u8
        ]
    }
}

fn render() {
    let width = 1024;
    let height = 768;
    let mut framebuffer : Vec<Pixel> = Vec::new();

    for j in 0..height {
        for i in 0..width {
            framebuffer.push(Pixel(j as f32 / (height as f32), i as f32 / (width as f32), 0.0))
        }
    }

    let mut out = File::create("out.ppm").unwrap();

    out.write_fmt(format_args!("P6\n{} {}\n255\n", width, height));

    for pixel in framebuffer.iter() {
        out.write(&pixel.as_bytes());
    }
}




fn main() {
    render();
}
