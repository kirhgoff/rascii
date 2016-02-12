mod rascii;

extern crate image;

use std::path::Path;
//use std::vec::Vec;
use std::env;
//use std::char;

use image::GenericImage;
use image::FilterType;

//use rascii::Slice;
use rascii::SlicedImage;

fn main() {
    //Some constants
    let ascii_start = 32;
    let ascii_end = 126;
    let ascii_count = ascii_end - ascii_start;
    let ascii_width = 40;

    println!("-----------------------------------------------------");
    println!("rascii - ascii image converter v2.0");
    println!("-----------------------------------------------------");

    //Slice source image
    let path = env::args().nth(1).unwrap();
    let img = image::open(&Path::new(&path)).unwrap().grayscale();
    let source_sliced = SlicedImage::new_source(&img, ascii_width);

    //Slice ascii map
    let ascii_map = image::open(&Path::new("asciimap.png")).unwrap().grayscale();
    let ascii_map = ascii_map.resize_exact(
      source_sliced.slice_width, 
      source_sliced.slice_height*ascii_count, 
      FilterType::Triangle
    );
    let ascii_sliced = SlicedImage::new(&ascii_map, 1, ascii_count);
    
    println!("original image size ({}, {})", source_sliced.width, source_sliced.height);
    println!("image slice size ({}, {})", source_sliced.slice_width, source_sliced.slice_height);
    println!("ascii map size ({}, {})", ascii_sliced.width, ascii_sliced.height);
    println!("ascii slice size ({}, {})", ascii_sliced.slice_width, ascii_sliced.slice_height);

/*    let mut ascii:Vec<u32> = Vec::new();

    //Iterate over every cell
    for column in 0..ascii_height {
      for row in 0..ascii_width {
        let mut best_char = 0;
        let mut weight = std::i32::MAX;
        for index in 0..ascii_count {
          let mut current:i32 = 0;

          let x_shift = row*cell_width;
          let y_shift = column*cell_height;

          for x in x_shift..(x_shift + cell_width) {
            for y in y_shift..(y_shift + cell_height) {

              let source_pixel = img.get_pixel(x, y);
              let char_pixel = ascii_map.get_pixel(x - x_shift, index*cell_height + y - y_shift);
              current += (char_pixel.data[0] as i32 - source_pixel.data[0] as i32).abs();

            }
          }
          if current < weight {
            weight = current;
            best_char = index;
          }
        }
        ascii.push(ascii_start + best_char);
      }
    }

    println!("-----------------------------------------------------");
    let mut counter = 1;
    for x in &ascii {
      print!("{}", char::from_u32(*x).unwrap().to_string());
      if counter % ascii_width == 0 {
        print!("\n");
      }
      counter += 1;
    } 
    println!("\n-----------------------------------------------------");
    */
    println!("Done!");
}


