use std::str::FromStr;
use xsd_types::lexical::{UnsignedInteger, UnsignedIntegerBuf};

#[test]
fn test_valid_values() {
    // Test some valid unsigned integer values
    assert!(UnsignedInteger::new("0").is_ok());
    assert!(UnsignedInteger::new("1").is_ok());
    assert!(UnsignedInteger::new("42").is_ok());
    assert!(UnsignedInteger::new("123456789").is_ok());
    assert!(UnsignedInteger::new("+42").is_ok());
    assert!(UnsignedInteger::new("+0").is_ok());
}

#[test]
fn test_invalid_values() {
    // Test some invalid unsigned integer values
    assert!(UnsignedInteger::new("-1").is_err());
    assert!(UnsignedInteger::new("-42").is_err());
    assert!(UnsignedInteger::new("1.5").is_err());
    assert!(UnsignedInteger::new("abc").is_err());
    assert!(UnsignedInteger::new("").is_err());
}

#[test]
fn test_canonical_form() {
    // Test canonical form removes leading zeros
    assert_eq!(UnsignedInteger::new("0").unwrap().canonical().as_str(), "0");
    assert_eq!(UnsignedInteger::new("00").unwrap().canonical().as_str(), "0");
    assert_eq!(UnsignedInteger::new("01").unwrap().canonical().as_str(), "1");
    assert_eq!(UnsignedInteger::new("001").unwrap().canonical().as_str(), "1");
    assert_eq!(UnsignedInteger::new("+01").unwrap().canonical().as_str(), "1");
    assert_eq!(UnsignedInteger::new("+001").unwrap().canonical().as_str(), "1");
}

#[test]
fn test_equality() {
    // Test equality with different representations
    assert_eq!(UnsignedInteger::new("42").unwrap(), UnsignedInteger::new("42").unwrap());
    assert_eq!(UnsignedInteger::new("042").unwrap(), UnsignedInteger::new("42").unwrap());
    assert_eq!(UnsignedInteger::new("+42").unwrap(), UnsignedInteger::new("42").unwrap());
    assert_eq!(UnsignedInteger::new("+042").unwrap(), UnsignedInteger::new("42").unwrap());
    
    // Test inequality
    assert_ne!(UnsignedInteger::new("42").unwrap(), UnsignedInteger::new("43").unwrap());
}

#[test]
fn test_comparison() {
    // Test ordering
    assert!(UnsignedInteger::new("42").unwrap() < UnsignedInteger::new("100").unwrap());
    assert!(UnsignedInteger::new("100").unwrap() > UnsignedInteger::new("42").unwrap());
    assert!(UnsignedInteger::new("42").unwrap() <= UnsignedInteger::new("42").unwrap());
    assert!(UnsignedInteger::new("42").unwrap() >= UnsignedInteger::new("42").unwrap());
}

#[test]
fn test_owned_type() {
    // Test the owned type
    let buf = UnsignedIntegerBuf::from_str("42").unwrap();
    assert_eq!(buf.as_str(), "42");
    
    // Test zero and one constructors
    let zero = UnsignedIntegerBuf::zero();
    assert_eq!(zero.as_str(), "0");
    
    let one = UnsignedIntegerBuf::one();
    assert_eq!(one.as_str(), "1");
}

#[test]
fn test_conversion() {
    // Test conversion to primitive types
    let uint = UnsignedInteger::new("42").unwrap();
    
    let u8_val: Result<u8, _> = u8::try_from(uint);
    assert!(u8_val.is_ok());
    if let Ok(val) = u8_val {
        assert_eq!(val, 42u8);
    }
    
    let u16_val: Result<u16, _> = u16::try_from(UnsignedInteger::new("42").unwrap());
    assert!(u16_val.is_ok());
    if let Ok(val) = u16_val {
        assert_eq!(val, 42u16);
    }
    
    let u32_val: Result<u32, _> = u32::try_from(UnsignedInteger::new("42").unwrap());
    assert!(u32_val.is_ok());
    if let Ok(val) = u32_val {
        assert_eq!(val, 42u32);
    }
    
    let u64_val: Result<u64, _> = u64::try_from(UnsignedInteger::new("42").unwrap());
    assert!(u64_val.is_ok());
    if let Ok(val) = u64_val {
        assert_eq!(val, 42u64);
    }
    
    // Test overflow
    let large = UnsignedInteger::new("300").unwrap();
    let u8_large: Result<u8, _> = u8::try_from(large);
    assert!(u8_large.is_err());
}

#[test]
fn test_is_positive_and_is_zero() {
    // Test is_positive
    assert!(UnsignedInteger::new("1").unwrap().is_positive());
    assert!(UnsignedInteger::new("42").unwrap().is_positive());
    assert!(!UnsignedInteger::new("0").unwrap().is_positive());
    
    // Test is_zero
    assert!(UnsignedInteger::new("0").unwrap().is_zero());
    assert!(UnsignedInteger::new("+0").unwrap().is_zero());
    assert!(UnsignedInteger::new("00").unwrap().is_zero());
    assert!(!UnsignedInteger::new("1").unwrap().is_zero());
}

#[test]
fn test_sign() {
    use xsd_types::lexical::Sign;
    
    assert_eq!(UnsignedInteger::new("42").unwrap().sign(), Sign::Positive);
    assert_eq!(UnsignedInteger::new("0").unwrap().sign(), Sign::Zero);
}