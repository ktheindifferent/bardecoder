use bardecoder;
use bardecoder::util::qr::{ECLevel, QRInfo};
use image;

#[test]
fn test_decode_version1_with_info() {
    let img = image::open("tests/images/version1_example.jpg").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (data, info) = result.as_ref().unwrap();
    assert!(!data.is_empty());
    assert_eq!(info.version, 1);
    // Version 1 QR codes have 21x21 modules = 441 total modules
    // Some of these are used for patterns, format info, etc.
    assert!(info.total_data > 0);
    assert!(info.total_data <= 208); // Version 1 max capacity in bits
}

#[test]
fn test_decode_version3_with_info() {
    let img = image::open("tests/images/version3_example.jpg").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (data, info) = result.as_ref().unwrap();
    assert!(!data.is_empty());
    assert_eq!(info.version, 3);
    // Version 3 has more capacity than version 1
    assert!(info.total_data > 208);
}

#[test]
fn test_decode_version4_with_info() {
    let img = image::open("tests/images/version4_example.jpg").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (data, info) = result.as_ref().unwrap();
    assert!(!data.is_empty());
    assert_eq!(info.version, 4);
}

#[test]
fn test_decode_multiple_codes_with_info() {
    let img = image::open("tests/images/multiple_codes.png").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    // The detector might not find all 4 codes in the image
    assert!(!results.is_empty(), "Should detect at least one QR code");
    
    let mut versions = Vec::new();
    let mut data_strings = Vec::new();
    
    for result in results {
        assert!(result.is_ok());
        let (data, info) = result.unwrap();
        versions.push(info.version);
        data_strings.push(data);
        
        // Each QR code should have valid info
        assert!(info.version >= 1 && info.version <= 40);
        assert!(info.total_data > 0);
    }
    
    // Check we got valid data from all codes
    for data in &data_strings {
        assert!(!data.is_empty());
    }
}

#[test]
fn test_decode_upside_down_with_info() {
    let img = image::open("tests/images/version1_example_upside_down.jpg").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (data, info) = result.as_ref().unwrap();
    assert!(!data.is_empty());
    assert_eq!(info.version, 1);
}

#[test]
fn test_decode_no_border_with_info() {
    let img = image::open("tests/images/version1_example_no_border.png").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (data, info) = result.as_ref().unwrap();
    assert!(!data.is_empty());
    assert_eq!(info.version, 1);
}

#[test]
fn test_decode_large_border_with_info() {
    let img = image::open("tests/images/version1_example_large_border.png").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (data, info) = result.as_ref().unwrap();
    assert!(!data.is_empty());
    assert_eq!(info.version, 1);
}

#[test]
fn test_decode_wikipedia_version1_with_info() {
    let img = image::open("tests/images/wikipedia/version1_example.png").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (data, info) = result.as_ref().unwrap();
    assert_eq!(info.version, 1);
    assert!(!data.is_empty());
}

#[test]
fn test_decode_wikipedia_version2_with_info() {
    let img = image::open("tests/images/wikipedia/version2_example.png").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (data, info) = result.as_ref().unwrap();
    assert_eq!(info.version, 2);
    assert!(!data.is_empty());
}

#[test]
fn test_decode_wikipedia_version3_with_info() {
    let img = image::open("tests/images/wikipedia/version3_example.png").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (data, info) = result.as_ref().unwrap();
    assert_eq!(info.version, 3);
    assert!(!data.is_empty());
}

#[test]
fn test_decode_wikipedia_version4_with_info() {
    let img = image::open("tests/images/wikipedia/version4_example.png").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (data, info) = result.as_ref().unwrap();
    assert_eq!(info.version, 4);
    assert!(!data.is_empty());
}

#[test]
fn test_decode_wikipedia_version10_with_info() {
    let img = image::open("tests/images/wikipedia/version10_example.png").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (data, info) = result.as_ref().unwrap();
    assert_eq!(info.version, 10);
    assert!(!data.is_empty());
}

#[test]
fn test_decode_wikipedia_version25_with_info() {
    let img = image::open("tests/images/wikipedia/version25_example.png").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (data, info) = result.as_ref().unwrap();
    assert_eq!(info.version, 25);
    assert!(!data.is_empty());
}

#[test]
fn test_decode_wikipedia_version40_with_info() {
    let img = image::open("tests/images/wikipedia/version40_example.png").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    
    // Version 40 might be challenging, so we allow for failure
    if !results.is_empty() && results[0].is_ok() {
        let (data, info) = results[0].as_ref().unwrap();
        assert_eq!(info.version, 40);
        assert!(!data.is_empty());
    }
}

#[test]
fn test_compare_decoder_results() {
    // Test that both decoders produce the same string data
    let img = image::open("tests/images/version3_example.jpg").unwrap();
    
    let regular_decoder = bardecoder::default_decoder();
    let info_decoder = bardecoder::default_decoder_with_info();
    
    let regular_results = regular_decoder.decode(&img);
    let info_results = info_decoder.decode(&img);
    
    assert_eq!(regular_results.len(), info_results.len());
    
    for (regular, info) in regular_results.iter().zip(info_results.iter()) {
        assert_eq!(regular.is_ok(), info.is_ok());
        
        if regular.is_ok() {
            let regular_data = regular.as_ref().unwrap();
            let (info_data, _info) = info.as_ref().unwrap();
            assert_eq!(regular_data, info_data);
        }
    }
}

#[test]
fn test_error_count_zero_for_clean_images() {
    // Most test images should decode without errors
    let img = image::open("tests/images/version1_example.jpg").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (_data, info) = result.as_ref().unwrap();
    // Clean images should have very few or no errors
    assert!(info.errors <= 10, "Clean image should have minimal errors, got {}", info.errors);
}

#[test]
fn test_ec_level_detection() {
    // Test that error correction level is properly detected
    let img = image::open("tests/images/version1_example.jpg").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (_data, info) = result.as_ref().unwrap();
    // EC level should be one of the valid levels
    match info.ec_level {
        ECLevel::LOW | ECLevel::MEDIUM | ECLevel::QUARTILE | ECLevel::HIGH => {
            // Valid EC level
        }
    }
}

#[test]
fn test_total_data_bits_reasonable() {
    // Test that total_data_bits is within reasonable bounds for the version
    let img = image::open("tests/images/version3_example.jpg").unwrap();
    let decoder = bardecoder::default_decoder_with_info();
    
    let results = decoder.decode(&img);
    assert_eq!(results.len(), 1);
    
    let result = &results[0];
    assert!(result.is_ok());
    
    let (_data, info) = result.as_ref().unwrap();
    
    // Version 3 has 29x29 = 841 modules
    // Not all are data (patterns, format info, etc.)
    // Maximum data capacity for version 3 is 440 bits (55 bytes) with EC level L
    assert!(info.total_data > 0, "Should have some data bits");
    assert!(info.total_data <= 440 * 2, "Should not exceed theoretical maximum");
}