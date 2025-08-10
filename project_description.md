# Project Description - Bardecoder

## Project Summary
Bardecoder is a pure Rust library for detecting and decoding QR codes from images. It was forked to fix dependency issues with the image 0.23.14 crate. The library is designed to be modular, allowing different algorithms for detection, extraction, and decoding to be used interchangeably.

## Recent Work Completed

### Latest Commits (Most Recent First)
1. **Image Dependency Fix** (`0b2a68b`) - Fixed dependency issue with image crate
2. **Version Update** (`bdabf23`) - Set example version to latest released (Fix #30)
3. **Code Quality** (`5b9f365`) - Fixed clippy warnings
4. **QR Info Enhancement** (`e3f7ca0`) - Added total number of bits of data in QR to QRInfo (#26)
5. **New Decoder Feature** (`1d70870`) - Added QRDecoderWithInfo and default_decoder_with_info() for returning QR metadata alongside decoded data (#26)
6. **Generic Result Type** (`6c73708`) - Made RESULT type a generic parameter for flexibility
7. **Version 0.3.0** (`5b531a1`) - Bumped version to 0.3.0
8. **Extended QR Support** (`7e9585a`) - Added remaining QR Versions and EC Levels (#17)

## Key Features Added
- **QRDecoderWithInfo**: New decoder variant that returns both decoded data and metadata about the QR code
- **QRInfo Structure**: Contains metadata about decoded QR codes including:
  - Error correction level
  - Version information
  - Total number of data bits
- **Generic Result Types**: Decoders can now return different result types (String or tuple with metadata)
- **Extended QR Version Support**: Support for all QR code versions and error correction levels

## Test Status
✅ All tests passing (28 unit tests, 14 integration tests, 6 doc tests)
- Added new unit tests for QRDecoderWithInfo functionality
- Added tests for error counting in correction process

### Current Warnings to Address
- Deprecated method `to_luma()` should be replaced with `to_luma8()`
- Unused mutable variables in blockedmean.rs
- Non-local impl definitions from failure_derive macro

## Dependencies
- Core: image (0.23.14), log, failure, failure_derive, newtype_derive
- Build verified with Rust 1.40+

## Project Structure
- `/src/decode/` - QR code decoding logic
- `/src/detect/` - QR code detection algorithms
- `/src/extract/` - QR code extraction from images
- `/src/prepare/` - Image preprocessing
- `/src/util/` - Utility functions and data structures
- `/tests/` - Integration tests with sample QR images