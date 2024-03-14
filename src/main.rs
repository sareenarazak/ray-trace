mod vec3;
mod ray;

use crate::vec3::Color;
use std::fs::File;
use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    let mut image = File::create("img.ppm")?;

    let width = 256;
    let height = 256;

    image.write_all(format!("P3\n{width} {height}\n255\n").as_bytes())?;

    for j in 0..height {
        eprint!("\rScan lines remaining: {} ", height - j);
        io::stderr().flush()?;

        for i in 0..width {
            let pixel_color = Color::new(
                i as f64 / (width as f64 - 1.0),
                j as f64 / (height as f64 - 1.0),
                0.25,
            );
            image.write_all(pixel_color.write_color().as_bytes())?;
        }
    }

    eprint!("\rDone.                 \n");
    Ok(())
}
