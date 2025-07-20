use briny::prelude::*;

struct SensitiveStruct(u8);

impl Validate for SensitiveStruct {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.0 <= 127 {
            Ok(())
        } else {
            Err(ValidationError)
        }
    }
}

impl Pack for SensitiveStruct {
    fn pack(&self, mut out: PackRef) -> Result<(), ValidationError> {
        // displayed at end
        let buf = out.ref_mut(); // safely access inner &mut [u8]
        if buf.len() < 1 {
            return Err(ValidationError);
        }
        buf[0] = 3; // anything 0-255
        Ok(())
    }
}

fn main() -> Result<(), ValidationError> {
    let raw = UntrustedData::new(SensitiveStruct(42));
    let trusted = TrustedData::new(raw.trust()?.into_inner())?;

    let mut buf = [0u8; 1];
    trusted.pack(PackRef::new(&mut buf))?;

    println!("Packed: {:?}", buf);
    Ok(())
}
