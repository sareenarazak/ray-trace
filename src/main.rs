mod ray;
mod vec3;
mod hittable;

use crate::vec3::{Point3, Vec3};
use ray::Ray;
use std::fs::File;
use std::io::{self, Result, Write};
use vec3::Color;


fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0  {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();

        return 0.5 * Color::new(n.e0() + 1.0, n.e1() + 1.0, n.e2() + 1.0);
    }
    let unit_direction: Vec3 = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.e1() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64{
    let oc: Vec3 = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(ray.direction());
    let c = oc.length_squared() - (radius * radius);
    let discriminant = (half_b * half_b)  - (a * c);
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

pub fn main() -> Result<()> {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    // TODO: min value should be 1
    const IMAGE_HEIGHT: usize = IMAGE_WIDTH / ASPECT_RATIO as usize;

    let focal_length = 1.0;
    let viewport_height= 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH / IMAGE_HEIGHT) as f64;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;
    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    // Render
    let mut image = File::create("sky.ppm")?;
    image.write_all(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes())?;
    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScan lines remaining: {} ", IMAGE_HEIGHT - j);
        io::stderr().flush()?;

        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&ray);
            image.write_all(pixel_color.write_color().as_bytes())?;
        }
    }
    eprint!("\rDone.                 \n");
    Ok(())


}

// First image
// fn main() -> std::io::Result<()> {
//     let mut image = File::create("img.ppm")?;
//
//     let width = 256;
//     let height = 256;
//
//     image.write_all(format!("P3\n{width} {height}\n255\n").as_bytes())?;
//
//     for j in 0..height {
//         eprint!("\rScan lines remaining: {} ", height - j);
//         io::stderr().flush()?;
//
//         for i in 0..width {
//             let pixel_color = Color::new(
//                 i as f64 / (width as f64 - 1.0),
//                 j as f64 / (height as f64 - 1.0),
//                 0.25,
//             );
//             image.write_all(pixel_color.write_color().as_bytes())?;
//         }
//     }
//
//     eprint!("\rDone.                 \n");
//     Ok(())
// }
