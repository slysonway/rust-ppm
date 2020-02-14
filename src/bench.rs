extern crate test;

use super::*;
use test::Bencher;

// Bench for Pixel 

#[bench]
fn bench_create_pixel(b: &mut Bencher) {
  b.iter(|| Pixel::new(50, 100, 150));
}

#[bench]
fn bench_display_pixel(b: &mut Bencher) {
  let p = Pixel::new(255, 255, 255);
  b.iter(|| p.display());
}

#[bench]
fn bench_grayscale_pixel(b: &mut Bencher) {
  let p = Pixel::new(100, 100, 100);
  b.iter(|| p.grayscale());
}

// Bench for Not

#[bench]
fn bench_not_Pixel(b: &mut Bencher) {
  let p = Pixel::new(100, 100, 100);
  b.iter(|| p.not());
}

// Bench for conversion functions

#[bench]
fn bench_string_to_number32(b: &mut Bencher) {
  //let p = Pixel::new(100, 100, 100);
  let mut s = String::new();
  s = "1 23 45".to_string();
  b.iter(|| string_to_number32(&s));
}

#[bench]
fn bench_string_to_number8(b: &mut Bencher) {
  let mut s = String::new();
  s = "1 23 45".to_string();
  b.iter(|| string_to_number8(&s));
}
