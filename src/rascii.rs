extern crate image;

use std::vec::Vec;
use std::u8;
use std::u32;

use image::GenericImage;
use image::DynamicImage;

//Constants
const CONSOLE_RATIO:f32 = 1.8;

pub struct AsciiMapStrategy<'a> {
	ascii_map: &'a AsciiMap<'a>,
	//normalized: Vec<Vec<f32>>
}

impl<'a> AsciiMapStrategy<'a> {
	pub fn new(ascii_map: &'a AsciiMap) -> AsciiMapStrategy<'a> {
		AsciiMapStrategy {ascii_map: ascii_map}
	}

	pub fn char_for (&self, slice: &Slice) -> char {
		// TODO ugly - refactor
		let mut best_index:usize = 0;
		let mut best_weight = u32::MAX;

		for (index, char_slice) in self.ascii_map.slices.slices.iter().enumerate() {
			//println!("char_slice={} slice={}", char_slice.dots.len(), slice.dots.len());
			let it = slice.dots.iter().zip(char_slice.dots.iter());
			let weight = it.fold(0, |acc, x| acc + ((*x.0 as i32 - *x.1 as i32) as i32).abs() as u32);
			if weight < best_weight {
				best_index = index;
				best_weight = weight;
			}
		}
		let result:char = (self.ascii_map.char_start + best_index as u8) as char;
		//println!("best index={} result={} best_weight={}", best_index, result, best_weight);
		result
	}
}


//------------------------ Ascii Map ---------------------------

pub struct AsciiMap<'a> {
	slices: &'a SlicedImage,
	char_start: u8,
	char_end: u8
}

impl<'a> AsciiMap<'a> {
	pub fn new(slices:&SlicedImage, char_start:u8, char_end:u8) -> AsciiMap {
		AsciiMap {
			slices: slices, char_start: char_start, char_end: char_end
		}
	}
}

//------------------------ Sliced Image ---------------------------

pub struct Slice {
	cell_width: u32,
	cell_height: u32,
	col: u32,
	row: u32,
	dots: Vec<u8>
}

pub struct SlicedImage {
	pub slices: Vec<Slice>,
	pub width: u32,
	pub height: u32,
	pub slice_width: u32,
	pub slice_height: u32,
	pub cols: u32,
	pub rows: u32,
	pub min: u8,
	pub max: u8
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
					cell_width: cell_width,
					cell_height: cell_height,
					col: column,
					row: row,
					dots: dots
				};
        slices.push(slice);
  		}
  	}

  	let (min, max) = min_max(&slices);
  	SlicedImage {
  		slices: slices, 
  		slice_width: cell_width, 
  		slice_height: cell_height,
  		cols: cols,
  		rows: rows,
  		width: width,
  		height: height,
  		min: min,
  		max: max
  	}
  }
}

fn min_max (slices:&Vec<Slice>) -> (u8, u8) {
	slices.iter().flat_map(|slice|&slice.dots)
		.fold((u8::MAX, u8::MIN), |(min, max), value| {
			let mut new_min = &min;
			let mut new_max = &max;
			if value < &min {new_min = value;}
			if value > &max {new_max = value;}
			(*new_min, *new_max)
		})
}
