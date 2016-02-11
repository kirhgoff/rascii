extern crate image;

use std::path::Path;
use std::vec::Vec;
use std::env;

use image::GenericImage;

fn main() {
    let path = env::args().nth(1).unwrap();
    let img = image::open(&Path::new(&path)).unwrap();
    let img = img.grayscale();

    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    let (width, height) = img.dimensions();
    let ascii_width = 40;
    let ascii_height = ascii_width * height / (width as f32 * 1.8).round() as u32;
    let cell_width = width / ascii_width;
    let cell_height = height / ascii_height;

    let mut sums:Vec<i32> = Vec::new();

    //Iterate over every cell
    for column in 0..ascii_height {
      for row in 0..ascii_width {
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
    for x in &sums {
      if *x < min {
        min = *x;
      }
      if *x > max {
        max = *x;   
      }
    }

    //Print symbols
    let spread = max - min;
    let mut counter = 0;
    for x in &sums {
      let value = x - min;
      let percentage = value as f32/spread as f32;
      match percentage {
        r if r < 0.5 => print!("#"),
        _ => print!(" ")
      }
      if counter % ascii_width == 0 {
        print!("\n");
      }
      counter += 1;
    } 

    println!("\nDone!");
}


