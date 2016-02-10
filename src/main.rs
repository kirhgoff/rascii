extern crate image;

use std::path::Path;
use std::vec::Vec;

use image::GenericImage;

fn main() {
    let img = image::open(&Path::new("test.jpg")).unwrap();
    let img = img.grayscale();

    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    let (width, height) = img.dimensions();
    let asciiWidth = 30;
    let asciiHeight = asciiWidth * height / width;
    let mut sums:Vec<Vec<i32>> = Vec::new();

    for i in 0..asciiWidth {
      sums.push(Vec::new());
      for j in 0..asciiHeight {
        let mut row = sums.get(i as usize).unwrap();
        row.push(0);
      }
    }


    for x in 0..width {
      for y in 0..height {
        let pixel = img.get_pixel(x, y);
        println!("{} {} {}", pixel.data[0], pixel.data[1], pixel.data[2]);
      }
    }
}


