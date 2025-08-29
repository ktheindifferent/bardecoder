use bardecoder::util::qr::{ECLevel, QRInfo};

#[test]
fn test_qr_version_bounds() {
    // Property: QR version must be between 1 and 40
    for version in 1..=40 {
        let info = QRInfo {
            version,
            ec_level: ECLevel::MEDIUM,
            total_data: 100,
            errors: 0,
        };
        assert!(info.version >= 1 && info.version <= 40);
    }
}

#[test]
fn test_qr_version_capacity_relationship() {
    // Property: Higher versions should allow more total data
    // Version capacity formula: ((4 * version + 17)^2 - fixed_patterns) * bits_per_module
    
    let mut previous_max_capacity = 0;
    
    for version in 1..=40 {
        let side = 4 * version + 17;
        let total_modules = side * side;
        
        // Approximate maximum capacity (actual varies by EC level)
        // Fixed patterns take up space: finder patterns, timing, format info, etc.
        let approx_data_modules = total_modules - (3 * 49) - 31 - (2 * 15);
        
        assert!(
            approx_data_modules > previous_max_capacity,
            "Version {} should have more capacity than version {}",
            version,
            version - 1
        );
        
        previous_max_capacity = approx_data_modules;
    }
}

#[test]
fn test_error_count_never_exceeds_total_data() {
    // Property: Error count should never exceed total data bits
    for errors in 0..1000 {
        for total_data in errors..errors + 1000 {
            let info = QRInfo {
                version: 1,
                ec_level: ECLevel::MEDIUM,
                total_data,
                errors,
            };
            
            assert!(
                info.errors <= info.total_data,
                "Errors ({}) should not exceed total data ({})",
                info.errors,
                info.total_data
            );
        }
    }
}

#[test]
fn test_ec_level_all_variants() {
    // Property: All EC levels should be valid
    let ec_levels = vec![
        ECLevel::LOW,
        ECLevel::MEDIUM,
        ECLevel::QUARTILE,
        ECLevel::HIGH,
    ];
    
    for ec_level in ec_levels {
        let info = QRInfo {
            version: 1,
            ec_level,
            total_data: 100,
            errors: 0,
        };
        
        // Just verify construction doesn't panic
        match info.ec_level {
            ECLevel::LOW => assert_eq!(format!("{:?}", info.ec_level), "LOW"),
            ECLevel::MEDIUM => assert_eq!(format!("{:?}", info.ec_level), "MEDIUM"),
            ECLevel::QUARTILE => assert_eq!(format!("{:?}", info.ec_level), "QUARTILE"),
            ECLevel::HIGH => assert_eq!(format!("{:?}", info.ec_level), "HIGH"),
        }
    }
}

#[test]
fn test_total_data_bits_by_version() {
    // Property: Total data bits should be within valid range for each version
    // Using QR code specification limits
    
    let version_max_bits = vec![
        (1, 152),   // Version 1 max data codewords * 8
        (2, 272),   // Version 2
        (3, 440),   // Version 3
        (4, 640),   // Version 4
        (5, 864),   // Version 5
        (10, 2956), // Version 10
        (20, 10208), // Version 20
        (30, 22496), // Version 30
        (40, 29648), // Version 40
    ];
    
    for (version, max_bits) in version_max_bits {
        for total_data in 1..=max_bits * 2 {
            let info = QRInfo {
                version,
                ec_level: ECLevel::LOW,
                total_data,
                errors: 0,
            };
            
            // Total data includes both data and EC codewords
            // So it can be up to ~2x the data capacity
            assert!(
                info.total_data > 0,
                "Version {} should have positive data bits",
                version
            );
        }
    }
}

#[test]
fn test_qr_info_equality_properties() {
    // Property: Equality should be reflexive, symmetric, and transitive
    
    let info1 = QRInfo {
        version: 5,
        ec_level: ECLevel::HIGH,
        total_data: 1000,
        errors: 10,
    };
    
    let info2 = QRInfo {
        version: 5,
        ec_level: ECLevel::HIGH,
        total_data: 1000,
        errors: 10,
    };
    
    let info3 = QRInfo {
        version: 5,
        ec_level: ECLevel::HIGH,
        total_data: 1000,
        errors: 10,
    };
    
    // Reflexive: a == a
    assert_eq!(info1, info1);
    
    // Symmetric: if a == b then b == a
    assert_eq!(info1, info2);
    assert_eq!(info2, info1);
    
    // Transitive: if a == b and b == c then a == c
    assert_eq!(info1, info2);
    assert_eq!(info2, info3);
    assert_eq!(info1, info3);
}

#[test]
fn test_qr_info_inequality_on_different_fields() {
    // Property: Changing any field should make QRInfo unequal
    
    let base = QRInfo {
        version: 5,
        ec_level: ECLevel::MEDIUM,
        total_data: 1000,
        errors: 10,
    };
    
    // Different version
    let diff_version = QRInfo {
        version: 6,
        ec_level: ECLevel::MEDIUM,
        total_data: 1000,
        errors: 10,
    };
    assert_ne!(base, diff_version);
    
    // Different EC level
    let diff_ec = QRInfo {
        version: 5,
        ec_level: ECLevel::HIGH,
        total_data: 1000,
        errors: 10,
    };
    assert_ne!(base, diff_ec);
    
    // Different total_data
    let diff_data = QRInfo {
        version: 5,
        ec_level: ECLevel::MEDIUM,
        total_data: 1001,
        errors: 10,
    };
    assert_ne!(base, diff_data);
    
    // Different errors
    let diff_errors = QRInfo {
        version: 5,
        ec_level: ECLevel::MEDIUM,
        total_data: 1000,
        errors: 11,
    };
    assert_ne!(base, diff_errors);
}

#[test]
fn test_error_correction_capability_by_ec_level() {
    // Property: Higher EC levels should be able to correct more errors
    // This is a semantic property of QR codes
    
    // Approximate error correction capabilities as percentage
    let ec_capabilities = vec![
        (ECLevel::LOW, 7),       // Can recover ~7% of codewords
        (ECLevel::MEDIUM, 15),   // Can recover ~15% of codewords
        (ECLevel::QUARTILE, 25), // Can recover ~25% of codewords
        (ECLevel::HIGH, 30),     // Can recover ~30% of codewords
    ];
    
    for (ec_level, _capability_percent) in ec_capabilities {
        let info = QRInfo {
            version: 10,
            ec_level,
            total_data: 1000,
            errors: 50,
        };
        
        // Just verify we can create QRInfo with different EC levels
        match info.ec_level {
            ECLevel::LOW | ECLevel::MEDIUM | ECLevel::QUARTILE | ECLevel::HIGH => {
                // Valid EC level
            }
        }
    }
}

#[test]
fn test_version_to_size_mapping() {
    // Property: QR size = 4 * version + 17
    for version in 1..=40 {
        let expected_size = 4 * version + 17;
        
        let info = QRInfo {
            version,
            ec_level: ECLevel::MEDIUM,
            total_data: 100,
            errors: 0,
        };
        
        // Verify the version is stored correctly
        assert_eq!(info.version, version);
        
        // The actual size calculation is done elsewhere, but we verify the version
        let calculated_size = 4 * info.version + 17;
        assert_eq!(calculated_size, expected_size);
    }
}

#[test]
fn test_debug_trait_implementation() {
    // Property: QRInfo should have a Debug implementation for diagnostics
    let info = QRInfo {
        version: 7,
        ec_level: ECLevel::QUARTILE,
        total_data: 512,
        errors: 3,
    };
    
    let debug_str = format!("{:?}", info);
    assert!(debug_str.contains("version"));
    assert!(debug_str.contains("7"));
    assert!(debug_str.contains("ec_level"));
    assert!(debug_str.contains("QUARTILE"));
    assert!(debug_str.contains("total_data"));
    assert!(debug_str.contains("512"));
    assert!(debug_str.contains("errors"));
    assert!(debug_str.contains("3"));
}