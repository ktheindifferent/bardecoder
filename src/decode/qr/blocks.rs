use super::block_info;
use super::{BlockInfo, ECLevel, QRMask};

use crate::util::qr::{QRData, QRError};

#[allow(clippy::borrowed_box)] // QRMask is a trait, unsure how to solve
pub fn blocks(data: &QRData, level: &ECLevel, mask: &Box<QRMask>) -> Result<Vec<Vec<u8>>, QRError> {
    let bi = block_info(data.version, level)?;
    let mut codewords = Codewords::new(bi);
    let mut x = data.side - 1;
    let loc = alignment_location(data.version)?;

    loop {
        let y_range = y_range(x, data.side);

        for y in y_range {
            if is_data(data, &loc, x, y) {
                codewords.add_bit(mask(data, x, y));
            }

            if is_data(data, &loc, x - 1, y) {
                codewords.add_bit(mask(data, x - 1, y));
            }
        }

        if x == 1 {
            break;
        }

        x -= 2;
        if x == 6 {
            // skip timing pattern
            x = 5;
        }
    }

    let bi = block_info(data.version, level)?;
    let blocks = codewords.blocks();

    if blocks.len() != bi.len() {
        return Err(QRError {
            msg: format!("Expected {expected} blocks but found {found}", expected = bi.len(), found = blocks.len()),
        });
    }

    for (i, block) in blocks.iter().enumerate() {
        debug!("BLOCK {i}, CODEWORDS {len}", len = block.len());
    }

    for i in 0..blocks.len() {
        if bi[i].total_per as usize != blocks[i].len() {
            return Err(QRError {
                msg: format!(
                    "Expected {expected} codewords in block {block} but found {found}",
                    expected = bi[i].total_per,
                    block = i,
                    found = blocks[i].len()
                ),
            });
        }
    }

    Ok(blocks)
}

fn y_range(x: u32, side: u32) -> Box<dyn Iterator<Item = u32>> {
    let x = if x < 6 { x + 1 } else { x };
    if (i64::from(x) - i64::from(side) + 1) % 4 == 0 {
        Box::new((0..side).rev())
    } else {
        Box::new(0..side)
    }
}

fn is_data(data: &QRData, loc: &AlignmentLocation, x: u32, y: u32) -> bool {
    // timing patterns
    if x == 6 || y == 6 {
        return false;
    }

    // top left locator pattern
    if x < 9 && y < 9 {
        return false;
    }

    // top right locator pattern
    if x > data.side - 9 && y < 9 {
        return false;
    }

    // bottom left locator pattern
    if x < 9 && y > data.side - 9 {
        return false;
    }

    // top right version info
    if data.version >= 7 && x > data.side - 12 && y < 6 {
        return false;
    }

    // buttom left version info
    if data.version >= 7 && y > data.side - 12 && x < 6 {
        return false;
    }

    if x == data.side - 9 && y < 9 {
        return true;
    }

    if y == data.side - 9 && x < 9 {
        return true;
    }

    if is_alignment_coord(loc, x) && is_alignment_coord(loc, y) {
        return false;
    }

    true
}

/// Determines if a coordinate falls within an alignment pattern region.
/// 
/// This function checks if a given coordinate is part of either:
/// 1. A finder pattern (coordinates 4-8)
/// 2. An alignment pattern (based on the QR version's alignment location parameters)
///
/// # Bug Fix
/// The original code had an operator precedence bug: `coord - 4 % 6 <= 4`
/// Due to operator precedence, this was evaluated as `coord - (4 % 6) <= 4`,
/// which simplified to `coord - 4 <= 4`, or `coord <= 8`.
/// 
/// This accidentally produced the correct behavior for detecting finder patterns
/// at coordinates 4-8 when combined with the `coord >= 4` check.
/// 
/// The fix explicitly expresses this intent: `coord >= 4 && coord <= 8`
fn is_alignment_coord(loc: &AlignmentLocation, coord: u32) -> bool {
    // Check if coordinate falls within finder pattern regions (coordinates 4-8)
    if coord >= 4 && coord <= 8 {
        return true;
    }

    if coord < loc.start - 2 {
        return false;
    }

    if (coord - (loc.start - 2)) % loc.step <= 4 {
        return true;
    }

    false
}

fn alignment_location(version: u32) -> Result<AlignmentLocation, QRError> {
    match version {
        // no alignment patterns for version 1 but this saves some exception paths
        1 => Ok(AlignmentLocation::new(1000, 1000)),

        // only one alignment pattern for versions 2-6 but this saves some exception paths
        2 => Ok(AlignmentLocation::new(18, 1000)),
        3 => Ok(AlignmentLocation::new(22, 1000)),
        4 => Ok(AlignmentLocation::new(26, 1000)),
        5 => Ok(AlignmentLocation::new(30, 1000)),
        6 => Ok(AlignmentLocation::new(34, 1000)),

        // multiple alignment patterns
        7 => Ok(AlignmentLocation::new(22, 16)),
        8 => Ok(AlignmentLocation::new(24, 18)),
        9 | 14 => Ok(AlignmentLocation::new(26, 20)),
        10 => Ok(AlignmentLocation::new(28, 22)),
        11 | 17 => Ok(AlignmentLocation::new(30, 24)),
        12 | 25 => Ok(AlignmentLocation::new(32, 26)),
        13 | 20 => Ok(AlignmentLocation::new(34, 28)),
        15 => Ok(AlignmentLocation::new(26, 22)),
        16 => Ok(AlignmentLocation::new(26, 24)),
        18 => Ok(AlignmentLocation::new(30, 26)),
        19 | 40 => Ok(AlignmentLocation::new(30, 28)),
        36 => Ok(AlignmentLocation::new(24, 26)),
        _ => Err(QRError {
            msg: format!("Unknown version {version}"),
        }),
    }
}

#[derive(Debug)]
struct AlignmentLocation {
    start: u32,
    step: u32,
}

impl AlignmentLocation {
    fn new(start: u32, step: u32) -> AlignmentLocation {
        AlignmentLocation { start, step }
    }
}

struct Blocks {
    block_info: Vec<BlockInfo>,
    blocks: Vec<Vec<u8>>,

    round: usize,
    max_data_round: usize,
    block: usize,
    data_blocks: bool,
}

impl Blocks {
    fn new(block_info: Vec<BlockInfo>) -> Blocks {
        let mut blocks = vec![];
        let mut max_data_round: usize = 0;

        for bi in &block_info {
            if bi.data_per as usize > max_data_round {
                max_data_round = bi.data_per as usize;
            }

            blocks.push(vec![]);
        }

        Blocks {
            block_info,
            blocks,
            round: 0,
            max_data_round,
            block: 0,
            data_blocks: true,
        }
    }

    fn push(&mut self, byte: u8) {
        while self.data_blocks && self.round > self.block_info[self.block].data_per as usize - 1 {
            self.inc_count();
        }

        trace!("PUSHING {:08b} TO BLOCK {}", byte, self.block);

        self.blocks[self.block].push(byte);
        self.inc_count();
    }

    fn inc_count(&mut self) {
        if self.block == self.block_info.len() - 1 {
            self.block = 0;
            self.round += 1;

            if self.round == self.max_data_round {
                self.data_blocks = false;
            }
        } else {
            self.block += 1;
        }
    }
}

struct Codewords {
    current_byte: u8,
    bit_count: u8,
    blocks: Blocks,
}

impl Codewords {
    fn new(block_info: Vec<BlockInfo>) -> Codewords {
        Codewords {
            current_byte: 0,
            bit_count: 0,
            blocks: Blocks::new(block_info),
        }
    }

    fn add_bit(&mut self, bit: u8) {
        self.current_byte *= 2;
        self.current_byte += bit;
        self.bit_count += 1;

        if self.bit_count == 8 {
            self.blocks.push(self.current_byte);
            self.current_byte = 0;
            self.bit_count = 0;
        }
    }

    fn blocks(self) -> Vec<Vec<u8>> {
        self.blocks.blocks
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_alignment_locs() {
        let al = alignment_location(36).expect("Alignment location should exist for version 36");
        let side = 4 * 36 + 17;

        for x in 0..side {
            if (4..=8).contains(&x)
                || (22..=26).contains(&x)
                || (48..=52).contains(&x)
                || (74..=78).contains(&x)
                || (100..=104).contains(&x)
                || (126..=130).contains(&x)
                || (152..=156).contains(&x)
            {
                assert!(is_alignment_coord(&al, x));
            } else {
                assert!(!is_alignment_coord(&al, x));
            }
        }
    }

    #[test]
    fn test_is_alignment_coord_edge_cases() {
        // Test version 7 (first version with alignment patterns)
        let al = alignment_location(7).unwrap();
        
        // Test coordinates 0-3 (should be false - too low for finder pattern check)
        for coord in 0..4 {
            assert!(!is_alignment_coord(&al, coord), "coord {} should be false", coord);
        }
        
        // Test coordinates 4-8 (should be true - finder pattern region)
        for coord in 4..=8 {
            assert!(is_alignment_coord(&al, coord), "coord {} should be true (finder pattern)", coord);
        }
        
        // Test coordinates 9-19 (should be false - between patterns)
        for coord in 9..20 {
            assert!(!is_alignment_coord(&al, coord), "coord {} should be false", coord);
        }
        
        // Test alignment pattern positions for version 7 (start=22, step=16)
        // The second condition matches coordinates at start-2 and repeating by step
        for coord in 20..=24 {
            assert!(is_alignment_coord(&al, coord), "coord {} should be true (alignment pattern)", coord);
        }
    }

    #[test]
    fn test_is_alignment_coord_operator_precedence() {
        // This test specifically validates the fix for the operator precedence bug
        // The original bug was: coord - 4 % 6 <= 4
        // Which was evaluated as: coord - (4 % 6) <= 4, i.e., coord - 4 <= 4, i.e., coord <= 8
        // This accidentally worked correctly for detecting finder patterns at 4-8!
        // The fix explicitly checks for coord >= 4 && coord <= 8
        
        let al = alignment_location(10).unwrap();
        
        // Test critical values around the finder pattern
        assert!(!is_alignment_coord(&al, 3), "coord 3 should be false");
        assert!(is_alignment_coord(&al, 4), "coord 4 should be true (start of finder)");
        assert!(is_alignment_coord(&al, 8), "coord 8 should be true (end of finder)");
        assert!(!is_alignment_coord(&al, 9), "coord 9 should be false");
        assert!(!is_alignment_coord(&al, 10), "coord 10 should be false");
        
        // Test alignment pattern for version 10 (start=28, step=22)
        // Alignment patterns are at coordinates: start-2 to start+2
        for coord in 26..=30 {
            assert!(is_alignment_coord(&al, coord), "coord {} should be true (alignment)", coord);
        }
        assert!(!is_alignment_coord(&al, 31), "coord 31 should be false");
    }

    #[test]
    fn test_is_alignment_coord_various_versions() {
        // Test version 1 (no alignment patterns, only finder patterns)
        let al1 = alignment_location(1).unwrap();
        assert!(is_alignment_coord(&al1, 4), "v1: coord 4 in finder");
        assert!(is_alignment_coord(&al1, 8), "v1: coord 8 in finder");
        assert!(!is_alignment_coord(&al1, 9), "v1: coord 9 not in pattern");
        
        // Test version 10 (has alignment patterns)
        let al10 = alignment_location(10).unwrap();
        // Finder patterns at 4-8
        for coord in 4..=8 {
            assert!(is_alignment_coord(&al10, coord), "v10: coord {} should be in finder", coord);
        }
        // Check alignment pattern positions for version 10
        // Version 10 has alignment at positions calculated by the formula
        
        // Test version 25 (multiple alignment patterns)
        let al25 = alignment_location(25).unwrap();
        assert!(is_alignment_coord(&al25, 4), "v25: coord 4 in finder");
        assert!(is_alignment_coord(&al25, 8), "v25: coord 8 in finder");
        
        // Test version 40 (maximum version)
        let al40 = alignment_location(40).unwrap();
        assert!(is_alignment_coord(&al40, 4), "v40: coord 4 in finder");
        assert!(is_alignment_coord(&al40, 8), "v40: coord 8 in finder");
    }

    #[test]
    fn test_is_alignment_coord_pattern_spacing() {
        // Test that alignment patterns are detected at correct intervals
        let al = alignment_location(20).unwrap();
        
        // Finder pattern region (only 4-8 after fix)
        assert!(is_alignment_coord(&al, 4));
        assert!(is_alignment_coord(&al, 5));
        assert!(is_alignment_coord(&al, 6));
        assert!(is_alignment_coord(&al, 7));
        assert!(is_alignment_coord(&al, 8));
        
        // Gap after finder - should be false
        assert!(!is_alignment_coord(&al, 9));
        assert!(!is_alignment_coord(&al, 10));
        assert!(!is_alignment_coord(&al, 11));
        
        // Check that alignment patterns follow the expected step pattern
        // based on the AlignmentLocation structure
        let mut found_patterns = Vec::new();
        for coord in 0..100 {
            if is_alignment_coord(&al, coord) {
                found_patterns.push(coord);
            }
        }
        
        // Verify that patterns are grouped (5 consecutive coordinates per pattern)
        let mut i = 0;
        while i < found_patterns.len() {
            let start = found_patterns[i];
            // Each pattern should have 5 consecutive coordinates
            for j in 0..5 {
                if i + j < found_patterns.len() {
                    assert_eq!(found_patterns[i + j], start + j as u32,
                        "Pattern starting at {} should have consecutive coords", start);
                }
            }
            i += 5;
            // Skip to next pattern group
            while i < found_patterns.len() && found_patterns[i] == found_patterns[i-1] + 1 {
                i += 1;
            }
        }
    }
}
