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
    let ascii_width = 30;
    let ascii_height = ascii_width * height / width;
    let cell_width = width / ascii_width;
    let cell_height = height / ascii_height;

    let mut sums:Vec<i32> = Vec::new();

    //Iterate over every cell
    for row in 0..ascii_width {
      for column in 0..ascii_height {
        let mut current:i32 = 0;
        for x in (row*cell_width)..((row+1)*cell_width) {
          for y in (column*cell_height)..((column + 1)*cell_height) {
            let pixel = img.get_pixel(x, y);
            current += pixel.data[0] as i32;
          }
        }
        sums.push(current);
      }
    }

    //Normalize
    let mut min = std::i32::MAX;
    let mut max = std::i32::MIN;
    for x in sums {
      if x < min {
        min = x;
      }
      if x > max {
        max = x;   
      }
    }

    println!("Done!");
}


