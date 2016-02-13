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
use rascii::AsciiMap;
use rascii::AsciiMapStrategy;


fn main() {
    //Constants
    let ascii_start:u8 = 32; //Start from space
    let ascii_end:u8 = 126; //to the last printable char
    let ascii_width = 40;

    println!("-----------------------------------------------------");
    println!("rascii - ascii image converter v2.0");
    println!("-----------------------------------------------------");

    //Slice source image
    let path = env::args().nth(1).unwrap();
    let img = image::open(&Path::new(&path)).unwrap().grayscale();
    let source_sliced = SlicedImage::new_source(&img, ascii_width);

    //Slice ascii map
    let ascii_count = ascii_end - ascii_start;
    let ascii_map_image = image::open(&Path::new("asciimap.png")).unwrap().grayscale();
    let ascii_map_image = ascii_map_image.resize_exact(
      source_sliced.slice_width, 
      source_sliced.slice_height*ascii_count as u32, 
      FilterType::Triangle
    );
    let ascii_sliced = SlicedImage::new(&ascii_map_image, 1, ascii_count as u32);
    let ascii_map = AsciiMap::new(ascii_sliced, ascii_start, ascii_end);
    let strategy = AsciiMapStrategy::new(ascii_map);
    
    println!("original image size ({}, {})", source_sliced.width, source_sliced.height);
    println!("image slice size ({}, {})", source_sliced.slice_width, source_sliced.slice_height);
    println!("ascii map size ({}, {})", ascii_sliced.width, ascii_sliced.height);
    println!("ascii slice size ({}, {})", ascii_sliced.slice_width, ascii_sliced.slice_height);

    println!("-----------------------------------------------------");
    let mut counter = 1;
    for slice in &source_sliced.slices {
      let chr = strategy.char_for(&slice);
      print!("{}", chr);
      if counter % ascii_width == 0 {
        print!("\n");
      }
      counter += 1;
    } 
    println!("\n-----------------------------------------------------");
    println!("Done!");
}


