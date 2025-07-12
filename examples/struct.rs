use briny::prelude::*;

struct MySensitiveStruct(u8);

impl Validate for MySensitiveStruct {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.0 <= 127 {
            Ok(())
        } else {
            Err(ValidationError)
        }
    }
}

impl Pack for MySensitiveStruct {
    fn pack(&self, mut out: PackRef) -> Result<(), ValidationError> {
        let buf = out.ref_mut(); // safely access inner &mut [u8]
        if buf.len() < 1 {
            return Err(ValidationError);
        }
        // displayed at end
        buf[0] = 3; // anything 0-255
        Ok(())
    }
}

fn main() -> Result<(), ValidationError> {
    let raw = MySensitiveStruct(42);
    let trusted = TrustedData::new(raw)?; // This now works

    let mut buf = [0u8; 1];
    trusted.pack(PackRef::new(&mut buf))?;

    println!("Packed: {:?}", buf);
    Ok(())
}
