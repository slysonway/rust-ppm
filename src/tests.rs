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

        let p = Pixel{r:254, g:156, b:10};

        let p1 = Pixel{r:140, g:140, b:140};

        let p2 = Pixel{r:140, g:140, b:140};

        let res = PartialEq::eq(&p, &p1);
        let res2 = PartialEq::eq(&p2, &p1);

        assert_eq!(res, false);
        assert_eq!(res2, true);

    }
