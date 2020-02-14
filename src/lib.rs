// This crate is a library
#![crate_type = "lib"]
// This crate is named "ppm"
#![crate_name = "ppm"]

#![feature(test)]

#[cfg(test)]
mod bench;

mod tests;

use std::ops::Not;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

#[no_mangle]
pub extern fn dummy() -> i8 {
    42
}

#[no_mangle]
pub extern fn times(x: i8, y: i8) -> i8 {
    x * y
}

#[derive(Clone, Copy, PartialEq, Debug)]
/// A Pixel is represented here by three colors, each of them are u8.
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

/// Implements some functions for the struct Pixel
#[no_mangle]
impl Pixel {
    /// Constructor
    #[no_mangle]
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Pixel{r: red, g: green, b: blue}
    }

    /// Red getter
    pub fn red(&self) -> u8 {
        self.r
    }

    /// Green getter
    pub fn green(&self) -> u8 {
        self.g
    }

    /// Blue getter
    pub fn blue(&self) -> u8 {
        self.b
    }

    /// ToString for a pixel (Pixel)
    /// Return 
    /// the value r, g and b of self
    /// 
    /// # Arguments
    /// * `self` - a Pixel to be displayed
    /// 
    /// # Return
    /// * `String`- Print value RVB
    #[no_mangle]
    pub fn display(&self) -> String {
        format!("r: {}, g {}, b: {}", self.r, self.g, self.b)
    }
    
    /// Transform a RGB pixel (Pixel) to a grayscale pixel (between 0 and 255).
    /// Use an intermediate u32 var to calculate the average without u8 overflow.
    /// 
    /// # Arguments
    /// * `self` - a Pixel to be converted
    /// 
    /// # Return
    /// * `u8` - an integer corresponding to the converted Pixel
    /// 
    /// # Example
    /// If a Pixel(30, 28, 255) is passed as a parameter 
    /// the function will return 104.
    pub fn grayscale(&self) -> u8 {
        let average: u32 = (self.r as u32 + self.g as u32 + self.b as u32) / 3;
        average as u8
    }  
    
}

/// Impl block to implement the not() function
impl Not for Pixel {
    type Output = Pixel;
    /// Revert a pixel's color with !pixel
    /// 
    /// #Arguments
    /// * `self` - a Pixel to be reverted
    /// 
    /// #Return
    /// * `Self` - a Pixel reverted
    /// 
    /// #Example
    /// If a Pixel(100, 50, 75) is passed as a parametr 
    /// the function will return a Pixel(155, 205, 180).
    fn not(self) -> Self::Output {
        let mut p = self;
        p.r = 255 - self.r;
        p.g = 255 - self.g;
        p.b = 255 - self.b;
        p
    }
}

// impl PartialEq for Pixel {
//     /// Equals to determine if the two Pixel in parameters are equals.
//     /// Return true if self and other and equals
//     /// (the r, g and b of self are equals to the r, g and b of other)
//     /// 
//     /// # Arguments
//     /// * `self` - a Pixel to be compared
//     /// * `other` - a second Pixel to compare the first one
//     /// 
//     /// # Return
//     /// * `bool` - corresponding to the equality (or not) of the two arguments
//     fn eq(self, other: Pixel) -> bool {
//         self.r() == other.r() && self.g == other.g && self.b == other.b
//     }
// }

#[derive(Debug)]
/// An image is defined with a width, a height and a buffer containing all pixels.
pub struct Image {
    buffer: Vec<Pixel>,
    width: u32,
    height: u32
}

/// Used to call Image's functions
impl Image {
    /// Constructor
    pub fn new(buffer: Vec<Pixel>, height: u32, width: u32) -> Image {
        Image{buffer: buffer, height: height, width: width}
    }

    /// Width getter
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Height getter
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Buffer getter
    pub fn buffer(&self) -> &Vec<Pixel> {
        &self.buffer
    }

    /// Write tje image in path
    /// 
    /// # Arguments
    /// * `self` - Image to be save
    /// * `filename: &Path` - path to save image
    pub fn save(&self, filename: &Path) {
        let mut file = File::create(filename).expect("Cannot create the file");
        file.write_all(b"P3\n").expect("Cannot write P3");
        file.write_fmt(format_args!("{} {}\n", self.width, self.height)).expect("Cannot write width, height");
        file.write_all(b"255\n").expect("Cannot write max value for Pixels");
        for p in &self.buffer {
            file.write_fmt(format_args!("{} {} {}\n", p.r, p.g, p.b)).expect("Cannot write pixels");
        }
    }

    /// Invert the Colors of an Image using c.not()
    /// to invert each color of a pixel
    /// 
    /// # Arguments
    /// * image: Image - the image to be inverted
    /// # Return
    /// * Image - the image inverted
    pub fn invert(image: &Image) -> Image {
        let mut inv: Vec<Pixel> = Vec::new();
        for c in &image.buffer {
            inv.push(c.not());
        }

        return Image::new(inv, image.height, image.width);
    }
    
    /// Return a grayscale Image from a RGB Image.
    /// Each pixel of the grayscale Image is the sum of the RGB pixel / 3.
    /// 
    /// # Arguments
    /// * image:Image - The RGB Image to be converted
    /// # Return
    /// * Image - The grayscale Image converted
    pub fn grayscale(image: &Image) -> Image {
        let mut gray: Vec<Pixel> = Vec::new(); 

        for i in &image.buffer {
            let c: u8 = Pixel::grayscale(i);
            gray.push(Pixel::new(c, c, c));
        }

        return Image::new(gray, image.height, image.width);
    }

}

/// Create a new Image from a .ppm File
/// # Arguments
/// * filename: &Path - The path corresponding to the file to be read.
/// 
/// # Return
/// * Image - The Image created through the file read.
#[no_mangle]
pub fn new_with_file(filename: &Path) -> Option<Image> {
    let file = File::open(filename).expect("File cannot be open");
    let bufReader = BufReader::new(file);
    let mut height: u32 = 0;
    let mut width: u32 = 0;
    let mut buffer: Vec<Pixel> = Vec::new();

    for (i, line) in bufReader.lines().enumerate().by_ref() {
        // get the size of image
        if i == 1 {
            let size = string_to_number32(&line.unwrap());
            width = size[0];
            height = size[1];
        } else {
            let line: &String = &line.unwrap();
            if line.chars().next().unwrap() != '#' || i != 2 {
                let pixel = string_to_number8(&line);
                if pixel.len() == 3 {
                    let p = Pixel::new(pixel[0], pixel[1], pixel[2]);
                    buffer.push(p);
                }
            }
        }
    }
   Some(Image::new(buffer, height, width))
}

/// Transform a String with numbers to number 
/// a Vector<u32> for the picture size.
/// # Arguments
/// * line: &String - line of ppm file
/// 
/// # Return
/// * Vec<u32> - Array of u32
/// # Example :
/// * "1 23 45" passed as parameters will return Vec{1, 23, 45}. 
/// * "1 23 azer //& &é45" passed as parameters will return Vec{1, 23, 45}
pub fn string_to_number32(line: &String) -> Vec<u32> {
    let mut size: Vec<u32> = Vec::new();
    let mut n = String::new();
    for c in line.chars() {
        if c == '\0' || c == ' ' {
            if !n.is_empty() {
                size.push(n.parse().unwrap());
                n.clear();
            }
        } else if c.is_digit(10){
            n.push(c);
        }
    }
    // Add if a numer is at the end of the line
    if !n.is_empty() {
        size.push(n.parse().unwrap());
    }
    size
}

/// Transform a String with numbers to number 
/// a Vector<u8> for the picture size.
/// # Arguments
/// * line: &String - line of ppm file
/// 
/// # Return
/// * Vec<u8> - Array of u8
/// # Example :
/// * "1 23 45" passed as parameters will return Vec{1, 23, 45}. 
/// * "1 23 azer //& &é45" passed as parameters will return Vec{1, 23, 45}
pub fn string_to_number8(line: &String) -> Vec<u8> {
    let mut size: Vec<u8> = Vec::new();
    let mut n = String::new();
    for c in line.chars() {
       if c == '\0' || c == ' ' {
           if !n.is_empty() {
               size.push(n.parse().unwrap());
               n.clear();
           }
       } else if c.is_digit(10){
           n.push(c);
       }
   }
    // Add if a numer is at the end of the line
    if !n.is_empty() {
       size.push(n.parse().unwrap());
    }
    
    size
}


// extern crate libc;
// use libc::c_char;
// use libc::c_int;

// #[link(name = "ppma_io")]
// extern "C" {
//     fn ch_cap(ch: c_char) -> c_char;
//     fn ppma_read(input_name: *const c_char, xsize: *mut c_int, ysize: *mut c_int, rgb_max: *mut c_int, r: *mut c_int, g: *mut c_int, b: *mut c_int);
// }

// #[no_mangle]
// pub extern fn readPPM(car: c_char) -> c_char {
//     unsafe { ch_cap(car) }
// }