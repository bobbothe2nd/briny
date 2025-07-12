use briny::prelude::*;
use briny::trust::{ValidationError, TrustedData};
use briny::raw::{ByteBuf, Raw};

#[derive(Debug)]
struct MyData(u32);

impl Validate for MyData {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.0 < 100 {
            Ok(())
        } else {
            Err(ValidationError)
        }
    }
}

impl Raw<4> for MyData {
    fn from_bytes(bytes: [u8; 4]) -> Result<Self, ValidationError> {
        Ok(MyData(u32::from_le_bytes(bytes)))
    }

    fn to_bytes(&self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

#[test]
fn test_valid_trusted_data() {
    let raw = ByteBuf::<MyData, 4>::new(42u32.to_le_bytes());
    let parsed = raw.parse().expect("parse failed");
    let trusted = TrustedData::new(parsed).expect("validation failed");
    assert_eq!(trusted.get().0, 42);
}

#[test]
fn test_invalid_data_fails_validation() {
    let raw = ByteBuf::<MyData, 4>::new(123u32.to_le_bytes());
    let parsed = raw.parse().expect("parse failed");
    let trusted = TrustedData::new(parsed);
    assert!(trusted.is_err(), "Expected validation to fail");
}
