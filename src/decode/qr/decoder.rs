use super::super::Decode;

use crate::util::qr::{QRData, QRError, QRInfo};

/// Decode a QR code into a resulting String
///
/// This decoder will, in order:
/// * Determine QR Format information
/// * Extract the interleaved blocks of codewords
/// * Perform error correction
/// * Decode the blocks into a String
///
/// # Optimisation
/// The error correction process can be relatively expensive. This decoder has a fast detection of the existence of errors,
/// allowing to bypass the correction altogether if none exist. Users of this library are encouraged to provide high quality fault-free images,
/// speeding up the decoding process by not having to correct errors.
pub struct QRDecoder {}

impl QRDecoder {
    /// Construct a new QRDecoder
    pub fn new() -> QRDecoder {
        QRDecoder {}
    }
}

impl Decode<QRData, String, QRError> for QRDecoder {
    fn decode(&self, data: Result<QRData, QRError>) -> Result<String, QRError> {
        let qr_data = data?;

        let format = super::format::format(&qr_data)?;
        let blocks = super::blocks::blocks(&qr_data, &format.0, &format.1)?;
        let block_info = super::block_info(qr_data.version, &format.0)?;

        let mut all_blocks = vec![];

        for (block, bi) in blocks.into_iter().zip(block_info) {
            let corrected = super::correct::correct(block, &bi)?;

            for corr in corrected.iter().take(bi.data_per as usize) {
                all_blocks.push(*corr);
            }
        }

        debug!("TOTAL LENGTH {len}", len = all_blocks.len());

        let data = super::data::data(all_blocks, qr_data.version)?;
        Ok(data)
    }
}

/// Decode a QR code into a resulting String. It also includes some information about the decoded QR Code.
///
/// Functions the same as QRDecoder, apart from also returning some information about the decoded QR Code.
pub struct QRDecoderWithInfo {}

impl QRDecoderWithInfo {
    /// Construct a new QRDecoder
    pub fn new() -> QRDecoderWithInfo {
        QRDecoderWithInfo {}
    }
}

impl Decode<QRData, (String, QRInfo), QRError> for QRDecoderWithInfo {
    fn decode(&self, data: Result<QRData, QRError>) -> Result<(String, QRInfo), QRError> {
        let qr_data = data?;

        let format = super::format::format(&qr_data)?;
        let blocks = super::blocks::blocks(&qr_data, &format.0, &format.1)?;
        let block_info = super::block_info(qr_data.version, &format.0)?;

        let mut all_blocks = vec![];
        let mut total_errors = 0;

        for (block, bi) in blocks.into_iter().zip(block_info) {
            let (corrected, error_count) = super::correct::correct_with_error_count(block, &bi)?;

            for corr in corrected.iter().take(bi.data_per as usize) {
                all_blocks.push(*corr);
            }

            total_errors += error_count;
        }

        debug!("TOTAL LENGTH {len}", len = all_blocks.len());
        let total_data = (all_blocks.len() as u32) * 8;

        let data = super::data::data(all_blocks, qr_data.version)?;
        Ok((
            data,
            QRInfo {
                version: qr_data.version,
                ec_level: format.0,
                total_data,
                errors: total_errors,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::qr::ECLevel;

    #[test]
    fn test_qr_decoder_new() {
        let decoder = QRDecoder::new();
        // Just verify construction doesn't panic
        let _decoder_ref = &decoder;
    }

    #[test]
    fn test_qr_decoder_with_info_new() {
        let decoder = QRDecoderWithInfo::new();
        // Just verify construction doesn't panic
        let _decoder_ref = &decoder;
    }

    #[test]
    fn test_decode_invalid_data_error() {
        let decoder = QRDecoder::new();
        let error = QRError {
            msg: "Test error".to_string(),
        };
        let result = decoder.decode(Err(error.clone()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[test]
    fn test_decode_with_info_invalid_data_error() {
        let decoder = QRDecoderWithInfo::new();
        let error = QRError {
            msg: "Test error".to_string(),
        };
        let result = decoder.decode(Err(error.clone()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[test]
    fn test_qr_info_struct_fields() {
        let info = QRInfo {
            version: 7,
            ec_level: ECLevel::HIGH,
            total_data: 1024,
            errors: 5,
        };
        
        assert_eq!(info.version, 7);
        assert_eq!(info.ec_level, ECLevel::HIGH);
        assert_eq!(info.total_data, 1024);
        assert_eq!(info.errors, 5);
    }

    #[test]
    fn test_qr_info_equality() {
        let info1 = QRInfo {
            version: 3,
            ec_level: ECLevel::MEDIUM,
            total_data: 512,
            errors: 2,
        };
        
        let info2 = QRInfo {
            version: 3,
            ec_level: ECLevel::MEDIUM,
            total_data: 512,
            errors: 2,
        };
        
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_qr_info_inequality() {
        let info1 = QRInfo {
            version: 3,
            ec_level: ECLevel::MEDIUM,
            total_data: 512,
            errors: 2,
        };
        
        let info2 = QRInfo {
            version: 4,
            ec_level: ECLevel::MEDIUM,
            total_data: 512,
            errors: 2,
        };
        
        assert_ne!(info1, info2);
    }
}
