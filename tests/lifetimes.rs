use briny::prelude::*;
use briny::trust::{TrustedData, UntrustedData, ValidationError};

#[derive(Debug)]
struct Demo(u8);

impl Validate for Demo {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.0 < 10 {
            Ok(())
        } else {
            Err(ValidationError)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Str8([u8; 8]);

impl Raw<8> for Str8 {
    fn from_bytes(bytes: [u8; 8]) -> Result<Self, ValidationError> {
        core::str::from_utf8(&bytes).map_err(|_| ValidationError)?; // ensure valid UTF-8
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
fn test_trusted_lifetime_bound() {
    let value = Demo(3);

    // simulate untrusted input from some outer context
    let trusted: TrustedData<'_, Demo> = TrustedData::new(value).unwrap();

    assert_eq!(trusted.as_ref().0, 3);
}

#[test]
fn test_untrusted_lifetime_anchor() -> Result<(), ValidationError> {
    let result: TrustedData<'_, ByteBuf<_, 8>> = {
        // this string slice only lives inside this block
        let input = "hi there";
        let untrusted = UntrustedData::new(input);
        let untrusted_bytes: ByteBuf<Str8, 8> = ByteBuf::from_str(untrusted.as_ref())?;
        TrustedData::new(untrusted_bytes).unwrap()
    };

    // If we try to use `result` outside the block, it won't compile unless `'static` or `'a`
    assert_eq!(result.as_ref(), &ByteBuf::from_str("hi there")?);
    Ok(())
}
