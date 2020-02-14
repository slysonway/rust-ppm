extern crate test;

use super::*;
use test::Bencher;

#[bench]
fn bench_create_pixel(b: &mut Bencher) {
  b.iter(|| Pixel::new(50, 100, 150));
}