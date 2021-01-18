mod linalg;

use linalg::Vec3;
use linalg::Color;
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
    let img_width: i32 = 256;
    let img_height: i32 = 256;

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
            let pixel_color = Vec3 {
                e: [
                    (i as f64) / (img_width-1) as f64,
                    (j as f64) / (img_height-1) as f64,
                    0.25
                ]
            };
            Color::write_color(&mut file, pixel_color);
        }
    }
    println!("\nDone.");
}