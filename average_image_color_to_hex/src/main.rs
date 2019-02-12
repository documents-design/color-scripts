extern crate image;

use std::env;
use std::path::Path;
use image::RgbImage;

fn rgb_to_hex(color: (i64, i64, i64)) -> String {
  format!("{:2X}{:2X}{:2X}", color.0, color.1, color.2).to_string()
}

fn main() {
    
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };
    
    let im = image::open(&Path::new(&file)).unwrap();
    let rgb_im : RgbImage = im.to_rgb();
    let mut r : i64 = 0;
    let mut g : i64 = 0;
    let mut b : i64 = 0;
    let mut count : i64 = 0;

    for px in rgb_im.pixels() {
      r = r + px.data[0] as i64;
      g = g + px.data[1] as i64;
      b = b + px.data[2] as i64;
      count += 1;
    }
    
    let average : (i64, i64, i64) = (r / count, g / count, b / count);
    println!("#{}", rgb_to_hex(average));
}