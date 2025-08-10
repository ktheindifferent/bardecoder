# Bardecoder Todo List

## Immediate Tasks (High Priority)

### Code Quality & Maintenance
- [ ] Fix deprecated `to_luma()` method - replace with `to_luma8()` in blockedmean.rs:36
- [ ] Remove unnecessary mutable variables in blockedmean.rs (lines 69, 75)
- [ ] Fix identity operation warning in blocks.rs:122 (4 % 6 should be (coord - 4) % 6)
- [ ] Apply clippy fixes for format strings (use inline format variables)
- [ ] Fix needless borrows identified by clippy
- [ ] Fix empty line after doc comment in decode/mod.rs:39
- [ ] Fix manual Option::map implementation in linescan.rs:458
- [ ] Fix manual range contains check in chomp.rs:79

### Testing Enhancements
- [ ] Add unit tests for QRDecoderWithInfo::new()
- [ ] Add integration tests for QRDecoderWithInfo decoding with info
- [ ] Test correct_with_error_count() function for error counting
- [ ] Add tests for QRInfo structure population
- [ ] Test default_decoder_with_info() function
- [ ] Add benchmarks for new decoder with info vs regular decoder
- [ ] Test edge cases for error correction counting

### Documentation
- [ ] Add examples for using QRDecoderWithInfo in README
- [ ] Document QRInfo structure fields in detail
- [ ] Add performance comparison notes between decoders
- [ ] Create migration guide from QRDecoder to QRDecoderWithInfo

## Future Enhancements (Medium Priority)

### Feature Development
- [ ] Implement remaining QR code modes (currently only numeric, alphanumeric, byte modes)
- [ ] Add support for structured append mode
- [ ] Implement Kanji mode decoding
- [ ] Add FNC1 mode support
- [ ] Implement ECI (Extended Channel Interpretation) mode

### Performance Optimization
- [ ] Profile and optimize error correction algorithm
- [ ] Investigate parallel processing for multiple QR codes
- [ ] Optimize memory allocations in decode path
- [ ] Add caching for frequently used computations

### Error Handling
- [ ] Improve error messages with more context
- [ ] Add recovery strategies for partially damaged QR codes
- [ ] Implement confidence scoring for decoded results
- [ ] Add diagnostic information for failed decodes

## Long-term Goals (Low Priority)

### Architecture Improvements
- [ ] Consider migrating from failure to thiserror/anyhow
- [ ] Update to latest image crate version (currently using 0.23.14, latest is 0.25.x)
- [ ] Remove deprecated newtype_derive in favor of modern alternatives
- [ ] Add async/await support for batch processing

### Additional Barcode Support
- [ ] Research feasibility of adding Data Matrix support
- [ ] Consider adding Aztec code support
- [ ] Evaluate PDF417 implementation

### Developer Experience
- [ ] Add more debug image output options
- [ ] Create interactive debugging tools
- [ ] Improve builder pattern with compile-time validation
- [ ] Add property-based testing with proptest

## Completed Recently
- ✅ Added QRDecoderWithInfo for metadata extraction
- ✅ Implemented QRInfo structure with version, EC level, data bits
- ✅ Added error counting in correction process
- ✅ Made RESULT type generic
- ✅ Fixed image dependency issue (0.23.14)
- ✅ Added support for all QR versions and EC levels

## Notes
- Tests are currently passing but with warnings
- Consider addressing non-local impl warnings from failure_derive
- Debug image feature available with `debug-images` feature flag
- Benchmark feature available with `benchmark` feature flag