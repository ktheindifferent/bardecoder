# Bardecoder Project Documentation

## Project Overview
Bardecoder is a pure Rust library for detecting and decoding QR codes from images. Originally forked to fix dependency issues with the image crate (v0.23.14), it provides a robust, modular architecture for QR code processing.

## Core Purpose
- **Primary Function**: Detect and decode QR codes from images
- **Language**: 100% Rust
- **Design Philosophy**: Modular architecture allowing interchangeable algorithms
- **Use Cases**: QR code scanning, image processing, barcode detection

## Technical Architecture

### Processing Pipeline
```
Input Image → Prepare → Detect → Extract → Decode → Result
```

1. **Prepare Stage** (`src/prepare/`)
   - Converts images to grayscale
   - Applies noise reduction (BlockedMean algorithm)
   - Optimizes images for QR detection

2. **Detect Stage** (`src/detect/`)
   - LineScan algorithm for pattern recognition
   - Identifies QR code locations in images
   - Returns coordinate positions

3. **Extract Stage** (`src/extract/`)
   - Perspective correction
   - Alignment pattern handling
   - Produces normalized QR matrices

4. **Decode Stage** (`src/decode/`)
   - Reed-Solomon error correction
   - Supports QR versions 1-40
   - All error correction levels (L, M, Q, H)
   - Returns decoded string or metadata

### Key Components

#### Decoders
- `default_decoder()` - Returns decoded string
- `default_decoder_with_info()` - Returns string + QRInfo metadata
- `DecoderBuilder` - Customizable decoder construction

#### Data Structures
- `QRInfo` - Metadata about decoded QR codes:
  - Error correction level
  - Version information
  - Total data bits
- `Location` - QR code position coordinates
- `ECLevel` - Error correction levels enum

## Codebase Structure
```
/root/repo/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── decoder.rs          # Core decoder implementations
│   ├── decode/            # QR decoding logic
│   │   ├── mod.rs
│   │   └── qr/           # QR-specific decoding
│   │       ├── blocks.rs  # Block processing
│   │       ├── correct.rs # Error correction
│   │       ├── data.rs    # Data extraction
│   │       ├── decoder.rs # QR decoder
│   │       ├── format.rs  # Format information
│   │       ├── galois.rs  # Galois field math
│   │       └── mod.rs
│   ├── detect/            # QR detection algorithms
│   │   ├── linescan.rs   # Line scanning algorithm
│   │   └── mod.rs
│   ├── extract/          # QR extraction logic
│   │   └── qr/
│   ├── prepare/          # Image preprocessing
│   │   ├── blockedmean.rs # Noise reduction
│   │   └── mod.rs
│   └── util/             # Utility functions
│       ├── chomp.rs      # Data parsing
│       ├── point.rs      # Point/coordinate handling
│       └── qr.rs         # QR-specific utilities
├── tests/                # Integration tests
│   ├── image_tests.rs    # Main test suite
│   └── images/           # Test QR code images
├── benches/              # Performance benchmarks
│   └── image_benches.rs
├── Cargo.toml           # Rust package manifest
├── README.md            # User documentation
├── overview.md          # Architecture overview
├── project_description.md # Recent work summary
└── todo.md              # Development roadmap
```

## Dependencies
```toml
[dependencies]
image = "0.23.14"      # Image processing
log = "0.4"           # Logging framework
failure = "0.1"       # Error handling
failure_derive = "0.1" # Error derive macros
newtype_derive = "0.1" # Newtype pattern macros
```

## Features
- `default` - Standard functionality
- `debug-images` - Outputs debug images to `/tmp/bardecoder-debug-images`
- `fail-on-warnings` - Treats warnings as errors (CI/CD)
- `benchmark` - Enables performance benchmarks

## Supported QR Code Features
✅ **Versions**: 1-40
✅ **Error Correction**: L, M, Q, H levels
✅ **Modes**: Numeric, Alphanumeric, Byte
✅ **Orientations**: Including upside-down
✅ **Borders**: With or without
✅ **Multiple Codes**: Per image

❌ **Not Yet Supported**:
- Kanji mode
- Structured append
- FNC1 mode
- ECI mode

## Testing Strategy
- **Unit Tests**: 28 tests for core algorithms
- **Integration Tests**: 14 tests with real QR images
- **Doc Tests**: 6 documentation examples
- **Test Images**: Wikipedia examples, various versions
- **Benchmarks**: Performance testing available

## Performance Guidelines
- **Optimal Resolution**: 400x300 to 800x600 pixels
- **Best Practices**:
  - Keep QR codes centered
  - Minimize image resolution for speed
  - Error-free codes exit early (optimization)
- **Debug Mode**: Use sparingly (generates many images)

## Current Development Status

### Recent Improvements
- Added `QRDecoderWithInfo` for metadata extraction
- Generic result types support
- Fixed image crate dependency issues
- Extended QR version support

### Known Issues
- Deprecated `to_luma()` needs replacement with `to_luma8()`
- Unused mutable variables in blockedmean.rs
- Non-local impl warnings from failure_derive
- Manual implementations that could use standard library methods

### Active Development Areas
See `todo.md` for detailed task list including:
- Code quality improvements
- Testing enhancements
- Documentation updates
- Feature development
- Performance optimization

## Build Instructions
```bash
# Standard build
cargo build --release

# With debug images
cargo build --features debug-images

# Run tests
cargo test

# Run benchmarks
cargo bench --features benchmark
```

## Usage Examples

### Quick Start
```rust
use bardecoder;
let img = image::open("qr_code.png").unwrap();
let decoder = bardecoder::default_decoder();
let results = decoder.decode(&img);
```

### With Metadata
```rust
let decoder = bardecoder::default_decoder_with_info();
let results = decoder.decode(&img);
for (data, info) in results {
    println!("Data: {}", data.unwrap());
    println!("Version: {}, EC: {:?}", info.version, info.ec_level);
}
```

### Custom Configuration
```rust
use bardecoder::prepare::BlockedMean;
let mut builder = bardecoder::default_builder();
builder.prepare(Box::new(BlockedMean::new(7, 9)));
let decoder = builder.build();
```

## Contributing Guidelines
- Submit issues for QR codes that fail to decode
- Include test images with bug reports
- Small fixes: Submit PR directly
- Large changes: Open issue first for discussion
- Follow existing code style and patterns

## License
MIT License

## Repository Information
- **GitHub**: https://github.com/pixelcoda/bardecoder
- **Crates.io**: https://crates.io/crates/bardecoder
- **Documentation**: https://docs.rs/bardecoder
- **Original Author**: Mark Arts
- **Current Maintainer**: Pixel Coda (Caleb Smith Woolrich)
- **Rust Version**: 1.40+