mod util;

use util::Vec3;
use util::Color;
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
            let percent_r = (i as f32) / (img_width-1) as f32;
            let percent_g = (j as f32) / (img_height-1) as f32;
            let percent_b = 0.25;

            let r = (percent_r * img_height as f32) as u32;
            let g = (percent_g * img_width as f32) as u32;
            let b = (percent_b * 255.999) as u32;

            writeln!(file, "{} {} {}", r, g, b).unwrap();
        }
    }
    println!("\nDone.");
}