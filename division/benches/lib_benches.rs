#![feature(test)]
extern crate division;
extern crate test;

mod naive_div {
    use division::div;

    #[bench]
    fn div_hundred(b: &mut ::test::Bencher) {
        b.iter(|| {
            let result = div(100, 3);
            ::test::black_box(result);
        });
    }

    #[bench]
    fn div_million(b: &mut ::test::Bencher) {
        b.iter(|| {
            let result = div(1_000_000, 3);
            ::test::black_box(result);
        });
    }

    #[bench]
    fn div_billion(b: &mut ::test::Bencher) {
        b.iter(|| {
            let result = div(1_000_000_000, 3);
            ::test::black_box(result);
        });
    }
}

mod faster_div {
    use division::div2;

    #[bench]
    fn div_hundred(b: &mut ::test::Bencher) {
        b.iter(|| {
            let result = div2(100, 3);
            ::test::black_box(result);
        });
    }

    #[bench]
    fn div_million(b: &mut ::test::Bencher) {
        b.iter(|| {
            let result = div2(1_000_000, 3);
            ::test::black_box(result);
        });
    }

    #[bench]
    fn div_billion(b: &mut ::test::Bencher) {
        b.iter(|| {
            let result = div2(1_000_000_000, 3);
            ::test::black_box(result);
        });
    }
}