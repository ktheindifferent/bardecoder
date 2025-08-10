# Bardecoder - High-Level Overview

## Purpose
Bardecoder is a pure Rust library for detecting and decoding QR codes from images. It provides a modular, extensible architecture that allows developers to swap different algorithms for each stage of the QR code processing pipeline.

## Architecture

### Processing Pipeline
The library follows a 4-stage processing pipeline:

```
Image → Prepare → Detect → Extract → Decode → Result
```

1. **Prepare Stage** (`prepare/`)
   - Converts input images to grayscale
   - Applies preprocessing algorithms (e.g., BlockedMean for noise reduction)
   - Prepares the image for QR code detection

2. **Detect Stage** (`detect/`)
   - Scans the prepared image for QR code patterns
   - Uses algorithms like LineScan to identify QR code locations
   - Returns a list of detected QR code positions

3. **Extract Stage** (`extract/`)
   - Extracts the QR code data from the detected locations
   - Handles perspective correction and alignment
   - Produces a normalized QR code matrix

4. **Decode Stage** (`decode/`)
   - Decodes the extracted QR code matrix into readable data
   - Handles error correction using Reed-Solomon codes
   - Supports all QR versions (1-40) and error correction levels

## Key Components

### Core Decoders
- **`default_decoder()`**: Returns decoded string data
- **`default_decoder_with_info()`**: Returns both data and metadata (version, EC level, data bits)

### Modular Design
Each stage uses trait-based abstraction:
- `Prepare<IMG, PREPD>` - Image preprocessing
- `Detect<PREPD>` - QR code detection
- `Extract<PREPD, LOC, DATA>` - QR code extraction
- `Decode<DATA, RESULT>` - QR code decoding

This allows users to:
- Use the default implementation for quick integration
- Customize individual stages with modified parameters
- Implement entirely custom algorithms for any stage

## Usage Patterns

### Quick Integration
```rust
let decoder = bardecoder::default_decoder();
let results = decoder.decode(&image);
```

### Custom Configuration
```rust
let mut builder = bardecoder::default_builder();
builder.prepare(Box::new(BlockedMean::new(7, 9)));
let decoder = builder.build();
```

### Advanced Customization
Users can implement their own algorithms for any stage by implementing the appropriate trait.

## Supported Features
- ✅ QR Code versions 1-40
- ✅ All error correction levels (L, M, Q, H)
- ✅ Multiple QR codes in single image
- ✅ Upside-down QR codes
- ✅ QR codes with/without borders
- ✅ Metadata extraction (version, EC level, data bits)

## Performance Considerations
- Optimized for images between 400x300 and 800x600 pixels
- Early exit for error-free QR codes
- Parallel processing support via rayon
- Debug image output available for troubleshooting

## Testing
Comprehensive test suite includes:
- Unit tests for core algorithms (Galois field math, error correction)
- Integration tests with real QR code images
- Wikipedia QR code examples for validation
- Support for various orientations and border conditions