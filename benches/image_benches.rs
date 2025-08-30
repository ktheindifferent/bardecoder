#![cfg_attr(feature = "benchmark", feature(test))]

#[cfg(all(feature = "benchmark", test))]
mod bench {
    extern crate bardecoder;
    extern crate image;
    extern crate test;

    use image::DynamicImage;

    use self::test::Bencher;

    #[bench]
    pub fn version1_example(b: &mut Bencher) {
        let img = image::open("tests/images/version1_example.jpg")
            .expect("Failed to open benchmark image: version1_example.jpg");
        bench_image(&img, b);
    }

    #[bench]
    pub fn version3_example2(b: &mut Bencher) {
        let img = image::open("tests/images/version3_example2.jpg")
            .expect("Failed to open benchmark image: version3_example2.jpg");
        bench_image(&img, b);
    }

    #[bench]
    pub fn needs_alignment(b: &mut Bencher) {
        let img = image::open("tests/images/needs_alignment.jpg")
            .expect("Failed to open benchmark image: needs_alignment.jpg");
        bench_image(&img, b);
    }

    pub fn bench_image(image: &DynamicImage, b: &mut Bencher) {
        let decoder = bardecoder::default_decoder();

        b.iter(|| decoder.decode(image))
    }

    #[bench]
    pub fn version1_example_with_info(b: &mut Bencher) {
        let img = image::open("tests/images/version1_example.jpg").unwrap();
        bench_image_with_info(&img, b);
    }

    #[bench]
    pub fn version3_example2_with_info(b: &mut Bencher) {
        let img = image::open("tests/images/version3_example2.jpg").unwrap();
        bench_image_with_info(&img, b);
    }

    #[bench]
    pub fn needs_alignment_with_info(b: &mut Bencher) {
        let img = image::open("tests/images/needs_alignment.jpg").unwrap();
        bench_image_with_info(&img, b);
    }

    pub fn bench_image_with_info(image: &DynamicImage, b: &mut Bencher) {
        let decoder = bardecoder::default_decoder_with_info();

        b.iter(|| decoder.decode(image))
    }

    #[bench]
    pub fn compare_decoders_small_qr(b: &mut Bencher) {
        let img = image::open("tests/images/version1_example.jpg").unwrap();
        let regular_decoder = bardecoder::default_decoder();
        let info_decoder = bardecoder::default_decoder_with_info();
        
        b.iter(|| {
            let _ = regular_decoder.decode(&img);
            let _ = info_decoder.decode(&img);
        })
    }

    #[bench]
    pub fn compare_decoders_medium_qr(b: &mut Bencher) {
        let img = image::open("tests/images/version3_example.jpg").unwrap();
        let regular_decoder = bardecoder::default_decoder();
        let info_decoder = bardecoder::default_decoder_with_info();
        
        b.iter(|| {
            let _ = regular_decoder.decode(&img);
            let _ = info_decoder.decode(&img);
        })
    }

    #[bench]
    pub fn compare_decoders_multiple_qr(b: &mut Bencher) {
        let img = image::open("tests/images/multiple_codes.png").unwrap();
        let regular_decoder = bardecoder::default_decoder();
        let info_decoder = bardecoder::default_decoder_with_info();
        
        b.iter(|| {
            let _ = regular_decoder.decode(&img);
            let _ = info_decoder.decode(&img);
        })
    }

    #[bench]
    pub fn decoder_construction_regular(b: &mut Bencher) {
        b.iter(|| {
            let _ = bardecoder::default_decoder();
        })
    }

    #[bench]
    pub fn decoder_construction_with_info(b: &mut Bencher) {
        b.iter(|| {
            let _ = bardecoder::default_decoder_with_info();
        })
    }
}
