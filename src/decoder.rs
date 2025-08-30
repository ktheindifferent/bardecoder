use image::DynamicImage;
use image::GrayImage;


use crate::decode::{Decode, QRDecoder, QRDecoderWithInfo};
use crate::detect::{Detect, LineScan, Location};
use crate::extract::{Extract, QRExtractor};
use crate::prepare::{BlockedMean, Prepare};

use crate::util::qr::{QRData, QRError, QRInfo, QRLocation};

/// Error type for `DecoderBuilder`
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    /// The prepare component is required but was not provided
    #[error("Cannot build Decoder without Prepare component")]
    MissingPrepare,
    /// The detect component is required but was not provided
    #[error("Cannot build Decoder without Detect component")]
    MissingDetect,
    /// The QR extract and decode components are required but were not provided
    #[error("Cannot build Decoder without QR extract and decode components")]
    MissingQR,
}

/// Struct to hold logic to do the entire decoding
pub struct Decoder<IMG, PREPD, RESULT> {
    prepare: Box<dyn Prepare<IMG, PREPD>>,
    detect: Box<dyn Detect<PREPD>>,
    qr: ExtractDecode<PREPD, QRLocation, QRData, RESULT, QRError>,
}

impl<IMG, PREPD, RESULT> Decoder<IMG, PREPD, RESULT> {
    /// Do the actual decoding
    ///
    /// Logic is run in the following order:
    /// * prepare
    /// * detect
    /// * per detected code the associated extract and decode functions
    pub fn decode(&self, source: &IMG) -> Vec<Result<RESULT, QRError>> {
        let prepared = self.prepare.prepare(source);
        let locations = self.detect.detect(&prepared);

        if locations.is_empty() {
            return vec![];
        }

        let mut all_decoded = vec![];

        for location in locations {
            match location {
                Location::QR(qrloc) => {
                    let extracted = self.qr.extract.extract(&prepared, qrloc);
                    let decoded = self.qr.decode.decode(extracted);

                    all_decoded.push(decoded);
                }
            }
        }

        all_decoded
    }
}

/// Create a default Decoder
///
/// It will use the following components:
///
/// * prepare: `BlockedMean`
/// * detect: `LineScan`
/// * extract: `QRExtractor`
/// * decode: `QRDecoder`
///
/// This is meant to provide a good balance between speed and accuracy
///
/// # Panics
///
/// This function will panic if the default builder fails to build,
/// which should never happen as all components are provided.
#[must_use]
pub fn default_decoder() -> Decoder<DynamicImage, GrayImage, String> {
    default_builder()
        .build()
        .expect("Default decoder should always build successfully: all required components are provided")
}

/// Create a default Decoder (non-panicking version)
///
/// It will use the following components:
///
/// * prepare: BlockedMean
/// * detect: LineScan
/// * extract: QRExtractor
/// * decode: QRDecoder
///
/// This is meant to provide a good balance between speed and accuracy
///
/// # Errors
///
/// Returns `BuilderError` if the decoder fails to build,
/// though this should never happen as all components are provided.
pub fn try_default_decoder() -> Result<Decoder<DynamicImage, GrayImage, String>, BuilderError> {
    default_builder().build()
}

/// Create a default Decoder that also returns information about the decoded QR Code
///
/// It will use the following components:
///
/// * prepare: `BlockedMean`
/// * detect: `LineScan`
/// * extract: `QRExtractor`
/// * decode: `QRDecoderWithInfo`
///
/// This is meant to provide a good balance between speed and accuracy
///
/// # Panics
///
/// This function will panic if the default builder fails to build,
/// which should never happen as all components are provided.
#[must_use]
pub fn default_decoder_with_info() -> Decoder<DynamicImage, GrayImage, (String, QRInfo)> {
    default_builder_with_info()
        .build()
        .expect("Default decoder with info should always build successfully: all required components are provided")
}

/// Create a default Decoder that also returns information about the decoded QR Code (non-panicking version)
///
/// It will use the following components:
///
/// * prepare: BlockedMean
/// * detect: LineScan
/// * extract: QRExtractor
/// * decode: QRDecoderWithInfo
///
/// This is meant to provide a good balance between speed and accuracy
///
/// # Errors
///
/// Returns `BuilderError` if the decoder fails to build,
/// though this should never happen as all components are provided.
pub fn try_default_decoder_with_info() -> Result<Decoder<DynamicImage, GrayImage, (String, QRInfo)>, BuilderError> {
    default_builder_with_info().build()
}

/// Builder struct to create a Decoder
///
/// Required elements are:
///
/// * Prepare
/// * Detect
/// * Extract
/// * Decode
pub struct DecoderBuilder<IMG, PREPD, RESULT> {
    prepare: Option<Box<dyn Prepare<IMG, PREPD>>>,
    detect: Option<Box<dyn Detect<PREPD>>>,
    qr: Option<ExtractDecode<PREPD, QRLocation, QRData, RESULT, QRError>>,
}

impl<IMG, PREPD, RESULT> DecoderBuilder<IMG, PREPD, RESULT> {
    /// Constructor; all fields initialized as None
    pub fn new() -> DecoderBuilder<IMG, PREPD, RESULT> {
        DecoderBuilder {
            prepare: None,
            detect: None,
            qr: None,
        }
    }

    /// Set the prepare implementation for this Decoder
    pub fn prepare(
        &mut self,
        prepare: Box<dyn Prepare<IMG, PREPD>>,
    ) -> &mut DecoderBuilder<IMG, PREPD, RESULT> {
        self.prepare = Some(prepare);
        self
    }

    /// Set the detect implementation for this Decoder
    pub fn detect(
        &mut self,
        detect: Box<dyn Detect<PREPD>>,
    ) -> &mut DecoderBuilder<IMG, PREPD, RESULT> {
        self.detect = Some(detect);
        self
    }

    /// Set the extact and decode implementations for this Decoder for QR codes
    pub fn qr(
        &mut self,
        extract: Box<dyn Extract<PREPD, QRLocation, QRData, QRError>>,
        decode: Box<dyn Decode<QRData, RESULT, QRError>>,
    ) -> &mut DecoderBuilder<IMG, PREPD, RESULT> {
        self.qr = Some(ExtractDecode { extract, decode });
        self
    }

    /// Build actual Decoder
    ///
    /// # Errors
    ///
    /// Returns `BuilderError` if any of the required components are missing:
    /// - `BuilderError::MissingPrepare` - prepare component not set
    /// - `BuilderError::MissingDetect` - detect component not set
    /// - `BuilderError::MissingQR` - QR extract/decode components not set
    pub fn build(self) -> Result<Decoder<IMG, PREPD, RESULT>, BuilderError> {
        let prepare = self.prepare.ok_or(BuilderError::MissingPrepare)?;
        let detect = self.detect.ok_or(BuilderError::MissingDetect)?;
        let qr = self.qr.ok_or(BuilderError::MissingQR)?;

        Ok(Decoder {
            prepare,
            detect,
            qr,
        })
    }
}

/// Create a default `DecoderBuilder`
///
/// It will use the following components:
///
/// * prepare: `BlockedMean`
/// * locate: `LineScan`
/// * extract: `QRExtractor`
/// * decode: `QRDecoder`
///
/// The builder can then be customised before creating the Decoder
#[must_use]
pub fn default_builder() -> DecoderBuilder<DynamicImage, GrayImage, String> {
    let mut db = DecoderBuilder::new();

    db.prepare(Box::new(BlockedMean::new(5, 7)));
    db.detect(Box::new(LineScan::new()));
    db.qr(Box::new(QRExtractor::new()), Box::new(QRDecoder::new()));

    db
}

/// Create a default `DecoderBuilder` that also returns information about the decoded QR Code
///
/// It will use the following components:
///
/// * prepare: `BlockedMean`
/// * locate: `LineScan`
/// * extract: `QRExtractor`
/// * decode: `QRDecoderWithInfo`
///
/// The builder can then be customised before creating the Decoder
#[must_use]
pub fn default_builder_with_info() -> DecoderBuilder<DynamicImage, GrayImage, (String, QRInfo)> {
    let mut db = DecoderBuilder::new();

    db.prepare(Box::new(BlockedMean::new(5, 7)));
    db.detect(Box::new(LineScan::new()));
    db.qr(
        Box::new(QRExtractor::new()),
        Box::new(QRDecoderWithInfo::new()),
    );

    db
}

struct ExtractDecode<PREPD, LOC, DATA, RESULT, ERROR> {
    extract: Box<dyn Extract<PREPD, LOC, DATA, ERROR>>,
    decode: Box<dyn Decode<DATA, RESULT, ERROR>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, GrayImage};

    #[test]
    fn test_builder_missing_prepare() {
        let mut builder: DecoderBuilder<DynamicImage, GrayImage, String> = DecoderBuilder::new();
        builder.detect(Box::new(LineScan::new()));
        builder.qr(Box::new(QRExtractor::new()), Box::new(QRDecoder::new()));
        
        let result = builder.build();
        assert!(result.is_err());
        match result {
            Err(BuilderError::MissingPrepare) => (),
            _ => panic!("Expected MissingPrepare error"),
        }
    }

    #[test]
    fn test_builder_missing_detect() {
        let mut builder: DecoderBuilder<DynamicImage, GrayImage, String> = DecoderBuilder::new();
        builder.prepare(Box::new(BlockedMean::new(5, 7)));
        builder.qr(Box::new(QRExtractor::new()), Box::new(QRDecoder::new()));
        
        let result = builder.build();
        assert!(result.is_err());
        match result {
            Err(BuilderError::MissingDetect) => (),
            _ => panic!("Expected MissingDetect error"),
        }
    }

    #[test]
    fn test_builder_missing_qr() {
        let mut builder: DecoderBuilder<DynamicImage, GrayImage, String> = DecoderBuilder::new();
        builder.prepare(Box::new(BlockedMean::new(5, 7)));
        builder.detect(Box::new(LineScan::new()));
        
        let result = builder.build();
        assert!(result.is_err());
        match result {
            Err(BuilderError::MissingQR) => (),
            _ => panic!("Expected MissingQR error"),
        }
    }

    #[test]
    fn test_builder_success() {
        let mut builder: DecoderBuilder<DynamicImage, GrayImage, String> = DecoderBuilder::new();
        builder.prepare(Box::new(BlockedMean::new(5, 7)));
        builder.detect(Box::new(LineScan::new()));
        builder.qr(Box::new(QRExtractor::new()), Box::new(QRDecoder::new()));
        
        let result = builder.build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_decoder_builds() {
        // This should not panic
        let _decoder = default_decoder();
    }

    #[test]
    fn test_default_decoder_with_info_builds() {
        // This should not panic
        let _decoder = default_decoder_with_info();
    }

    #[test]
    fn test_try_default_decoder() {
        let result = try_default_decoder();
        assert!(result.is_ok(), "try_default_decoder should succeed with all components");
        
        // Verify it returns the same type as default_decoder
        let _decoder: Decoder<DynamicImage, GrayImage, String> = result.expect("Should build decoder");
    }

    #[test]
    fn test_try_default_decoder_with_info() {
        let result = try_default_decoder_with_info();
        assert!(result.is_ok(), "try_default_decoder_with_info should succeed with all components");
        
        // Verify it returns the same type as default_decoder_with_info
        let _decoder: Decoder<DynamicImage, GrayImage, (String, QRInfo)> = result.expect("Should build decoder");
    }
}
