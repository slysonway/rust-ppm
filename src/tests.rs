extern crate test;

use super::*;

#[test]
fn test_dummy(){
    assert_eq!(dummy(), 42);
}

#[test]
fn test_on_pixel(){
    let p = Pixel{r:9, g:1, b:3};
    assert_eq!(Pixel::new(9, 1, 3), p);
}

#[test]
fn test_display_pixel() {
    let p = Pixel{r:200, g:0, b:0};
    p.display();
}

#[test]
fn test_equity(){

    let p = Pixel{r:60, g:60, b:60};

    let p1 = Pixel{r:90, g:90, b:90};

    let p2 = Pixel{r:90, g:90, b:90};

    let res = PartialEq::eq(&p, &p1);
    let res2 = PartialEq::eq(&p2, &p1);

    assert_eq!(res, false);
    assert_eq!(res2, true);

}

#[test]
fn an_image_inverted_equals_its_opposite() {
    let mut c: Vec<Pixel> = Vec::new();
    for _i in 0..6 {
        c.push(Pixel::new(255, 255, 255));
    }
    let i:Image = Image::new(c, 3, 2);

    let mut c2: Vec<Pixel> = Vec::new();
    for _i in 0..6 {
        c2.push(Pixel::new(0, 0, 0));
    }
    let i2:Image = Image::new(c2, 3, 2);

    assert_eq!(true, i.eq(Image::invert(&i2)));
}

#[test]
fn rgb_image_to_grayscale_image_work() {
    let mut c: Vec<Pixel> = Vec::new();
    for _i in 0..6 {
        c.push(Pixel::new(30, 128, 255));
    }
    let mut im:Image = Image::new(c, 3, 2);
    im = Image::grayscale(&im);

    let mut c2: Vec<Pixel> = Vec::new();
    for _i in 0..6 {
        c2.push(Pixel::new(137, 137, 137));
    }
    let im2:Image = Image::new(c2, 3, 2);

    assert_eq!(true, im.eq(im2));
}