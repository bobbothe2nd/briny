use briny::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Str8([u8; 8]);

impl Raw<8> for Str8 {
    fn from_bytes(bytes: [u8; 8]) -> Result<Self, ValidationError> {
        core::str::from_utf8(&bytes).map_err(|_| ValidationError)?;
        Ok(Str8(bytes))
    }

    fn to_bytes(&self) -> [u8; 8] {
        self.0
    }
}

impl Validate for Str8 {
    fn validate(&self) -> Result<(), ValidationError> {
        core::str::from_utf8(&self.0).map_err(|_| ValidationError)?;
        Ok(())
    }
}

#[test]
fn validate_valid_str() {
    let b = ByteBuf::<Str8, 8>::from_str("good").unwrap();
    let parsed = b.parse().unwrap();
    assert!(parsed.validate().is_ok());
}

#[test]
fn validate_invalid_utf8_bytes() {
    let invalid = [0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0];
    let b = ByteBuf::<Str8, 8>::new(invalid);
    let err = b.parse(); // calls from_bytes → validate
    assert!(err.is_err());
}

#[test]
fn validate_too_long_str() {
    let result = ByteBuf::<Str8, 8>::from_str("way-too-long");
    assert!(result.is_err());
}

#[test]
fn validate_exact_fit() {
    let b = ByteBuf::<Str8, 8>::from_str("12345678").unwrap();
    let parsed = b.parse().unwrap();
    assert_eq!(parsed.to_bytes(), *b.as_bytes());
}
