extern crate image;

use std::vec::Vec;

use image::GenericImage;
use image::DynamicImage;

//Constants
const CONSOLE_RATIO:f32 = 1.8;

pub struct Slice {
	pub width: u32,
	pub height: u32,
	pub col: u32,
	pub row: u32,
	pub dots: Vec<u8>
}

pub struct SlicedImage {
	pub slices: Vec<Slice>,
	pub width: u32,
	pub height: u32,
	pub slice_width: u32,
	pub slice_height: u32,
	pub cols: u32,
	pub rows: u32
}

pub struct AsciiMap {
	slices: SlicedImage,
	char_start: u8,
	char_end: u8
}

pub trait MatchingStrategy {

}

impl AsciiMap {
	pub fn new(slices:&SlicedImage, char_start:u8, char_end:u8) -> AsciiMap {
		AsciiMap {
			slices: slices, char_start: char_start, char_end: char_end
		}
	}

	pub fn char_for_slice (slice: &Slice, strategy: &MatchingStrategy) -> u8 {
		
	}
}

impl SlicedImage {
	pub fn new_source(img: &DynamicImage, cols:u32) -> SlicedImage {
		let (width, height) = img.dimensions();
	    let rows:u32 = ((cols * height) as f32 / (width as f32 * CONSOLE_RATIO)).round() as u32;
		SlicedImage::new(img, cols, rows)
	}

	pub fn new(img: &DynamicImage, cols:u32, rows:u32) -> SlicedImage {
        let (width, height) = img.dimensions();
        let cell_width = width / cols;
	    let cell_height = height / rows;

	    println!("creating source image ({}, {}) with slices({}, {}) slice size ({}, {})",
	    	width, height, cols, rows, cell_width, cell_height);  

		let mut slices:Vec<Slice> = Vec::new();

    	for row in 0..rows {
		    for column in 0..cols {

			    let mut dots:Vec<u8> = Vec::new();

				let x_shift = column*cell_width;
				let y_shift = row*cell_height;
				
				// println!("column={} row={} x_shift={} y_shift={}",
				//  	column, row, x_shift, y_shift);

				for x in x_shift..(x_shift + cell_width) {
					for y in y_shift..(y_shift + cell_height) {

						//println!("column={} row={} x={} y={}",
						//	column, row, x, y);
				  		let source_pixel = img.get_pixel(x, y).data[0];
				  		dots.push(source_pixel);
					}
				}
				
				let slice = Slice {
					width: cell_width,
					height: cell_height,
					col: column,
					row: row,
					dots: dots
				};
		        slices.push(slice);
    		}
    	}
    	SlicedImage {
    		slices: slices, 
    		slice_width: cell_width, 
    		slice_height: cell_height,
    		cols: cols,
    		rows: rows,
    		width: width,
    		height: height
    	}
    }
}