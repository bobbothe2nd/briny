use briny::pack::{Pack, PackRef, Unpack, UnpackBuf};
use briny::prelude::*;
use briny::raw::Raw;
use briny::trust::{TrustedData, ValidationError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Pack for MyData {
    fn pack(&self, mut out: PackRef<'_>) -> Result<(), ValidationError> {
        let buf = out.ref_mut();
        if buf.len() < 4 {
            return Err(ValidationError);
        }
        buf[..4].copy_from_slice(&self.to_bytes());
        Ok(())
    }
}

impl Unpack for MyData {
    fn unpack_and_validate(input: UnpackBuf<'_>) -> Result<TrustedData<'_, Self>, ValidationError> {
        let bytes: [u8; 4] = input.as_slice().try_into().map_err(|_| ValidationError)?;
        let parsed = Self::from_bytes(bytes)?;
        TrustedData::new(parsed)
    }
}

#[test]
fn test_pack_unpack_roundtrip() -> Result<(), Box<dyn core::error::Error>> {
    let original = TrustedData::new(MyData(42))?;

    let mut buffer = [0u8; 4];
    let out = PackRef::new(&mut buffer);
    original.pack(out)?;

    let input = UnpackBuf::new(&buffer);
    let unpacked = MyData::unpack_and_validate(input)?;

    assert_eq!(unpacked.as_ref(), original.as_ref());

    Ok(())
}

#[test]
fn roundtrip_pack_unpack_u32() -> Result<(), Box<dyn core::error::Error>> {
    use briny::raw::{ByteBuf, Raw};
    use briny::trust::{TrustedData, Validate, ValidationError};

    #[derive(Debug, Clone, Copy)]
    struct MyU32(u32);

    impl Raw<4> for MyU32 {
        fn from_bytes(bytes: [u8; 4]) -> Result<Self, ValidationError> {
            Ok(MyU32(u32::from_le_bytes(bytes)))
        }

        fn to_bytes(&self) -> [u8; 4] {
            self.0.to_le_bytes()
        }
    }

    impl Validate for MyU32 {
        fn validate(&self) -> Result<(), ValidationError> {
            if self.0 <= 10000 {
                Ok(())
            } else {
                Err(ValidationError)
            }
        }
    }

    let original = MyU32(1337);
    let raw = ByteBuf::<MyU32, 4>::new(original.to_bytes());
    let parsed = raw.parse()?;
    let trusted = TrustedData::new(parsed)?;
    assert_eq!(trusted.as_ref().0, 1337);

    Ok(())
}
