mod linalg;
mod geom; 

use geom::Ray;
use geom::util::ray_color;
use linalg::Vec3;
use linalg::Color;
use linalg::Point3;

use std::env;
use std::fs;
use std::io::Write;

fn main() {

    // open file to write img to
    let current_dir = env::current_dir().expect("Failed to read current_dir!");
    let filename = "img.ppm";
    let mut file = fs::OpenOptions::new()
                    .create(true) // new file
                    .append(true) // allow for constant appending
                    .open(filename) // open file as Result<File>
                    .unwrap(); // throw error if file creation fails

    println!("Created {}/{} to write image into.", current_dir.display(), filename);

    // image
    let aspect_ratio = 16.0/9.0;
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as u32;

    // background colors
    let color_1 = Color { e: [1.0, 1.0, 1.0] }; // white (ending color)
    let color_2 = Color { e: [0.5, 0.7, 1.0] }; // light blue (starting color)

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    
    let origin = Point3 { e: [0.0, 0.0, 0.0] };
    let horizontal = Vec3 { e: [viewport_width, 0.0, 0.0] };
    let vertical = Vec3 { e: [0.0, viewport_height, 0.0] };
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3 { e: [0.0, 0.0, focal_length] };
    
    // render image in PPM format
    // header
    writeln!(file, "P3").unwrap(); // colors are in ASCII
    writeln!(file, "{}", img_width).unwrap();
    writeln!(file, "{}", img_height).unwrap();
    writeln!(file, "255").unwrap(); // max color allowed by format

    // print RGB triplets
    // top left corner, green is 255 (max) and red is 0; green decreases down a column
    // bottom right corner, red is 255 and green is 0; red increases across a row
    // blue is constantly 25% of max value throughout img
    for j in (0..img_height-1).rev() { // iterate down column
        print!("\rScanlines remaining: {}", j);
        for i in 0..img_width { // iterate across row
            let u = i as f64 / (img_width - 1) as f64;
            let v = j as f64 / (img_height - 1) as f64;
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + u*horizontal + v*vertical - origin,
            };
            let pixel_color = ray_color(&r, &color_1, &color_2);
            Color::write_color(&mut file, pixel_color);
        }
    }
    println!("\nDone.");
}