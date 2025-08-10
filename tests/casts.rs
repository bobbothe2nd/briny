use briny::SafeMemory;
use briny::raw::{
    Pod,
    casting::{cast, from_bytes, slice_from_bytes, to_bytes},
};

#[test]
fn to_bytes_roundtrip_fuzz() {
    #[repr(C)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    struct Pair {
        a: u32,
        b: u64,
    }

    impl SafeMemory for Pair {}

    unsafe impl Pod for Pair {}

    let inputs: &[Pair] = &[
        Pair { a: 0, b: 0 },
        Pair {
            a: u32::MAX,
            b: u64::MAX,
        },
        Pair {
            a: 0x12345678,
            b: 0xCAFEBABEDEADBEEF,
        },
        Pair { a: 1, b: 2 },
        Pair {
            a: 0xFFFF_FFFF,
            b: 0,
        },
    ];

    for input in inputs {
        let bytes = to_bytes(input);
        let output = from_bytes::<Pair>(bytes).unwrap();
        assert_eq!(&output, input);
    }
}

#[test]
fn slice_from_bytes_misaligned_fuzz() {
    use core::mem::{align_of, size_of};

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    struct Word(u32);

    impl SafeMemory for Word {}

    unsafe impl Pod for Word {}

    let alignment = align_of::<Word>();
    let size = size_of::<Word>() * 10;

    let buf = [0u8; 64];
    for offset in 0..=16 {
        let slice = &buf[offset..offset + size];
        let result = slice_from_bytes::<Word>(slice);

        if offset % alignment == 0 {
            assert!(result.is_ok(), "offset {offset} should be aligned");
        } else {
            assert!(
                result.is_err(),
                "offset {offset} should be rejected as misaligned"
            );
        }
    }
}

#[test]
fn cast_struct_edge_fuzz() {
    #[repr(C, align(4))]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
    struct FourBytes {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
    }

    const _: () = {
        assert!(size_of::<FourBytes>() == size_of::<u32>());
        assert!(align_of::<FourBytes>() >= align_of::<u32>());
    };

    impl SafeMemory for FourBytes {}

    unsafe impl Pod for FourBytes {}

    for a in 0u8..=10 {
        for b in 0u8..=10 {
            for c in 0u8..=10 {
                for d in 0u8..=10 {
                    let orig = FourBytes { a, b, c, d };
                    let casted: u32 = cast(&orig).unwrap_or_default();
                    let back: FourBytes = cast(&casted).unwrap_or_default();
                    assert_eq!(orig, back);
                }
            }
        }
    }
}

#[test]
fn roundtrip_cast_fuzz() {
    #[repr(C, align(4))]
    #[derive(Copy, Clone, Debug, PartialEq, Default)]
    struct A {
        x: u16,
        y: u16,
    }

    impl SafeMemory for A {}

    unsafe impl Pod for A {}

    #[repr(C, align(4))]
    #[derive(Copy, Clone, Debug, PartialEq, Default)]
    struct B(u32);

    impl SafeMemory for B {}

    unsafe impl Pod for B {}

    const _: () = {
        assert!(size_of::<A>() == size_of::<B>());
        assert!(align_of::<A>() >= align_of::<B>());
    };

    for v in 0u32..100 {
        let b = B(v);
        let a = cast::<B, A>(&b).unwrap_or_default();
        let b2 = cast::<A, B>(&a).unwrap_or_default();
        assert_eq!(b, b2, "roundtrip failed for value {v}");
    }
}

#[test]
fn padding_and_alignment_edge_fuzz() {
    use core::mem::size_of;

    // aligned to u32, size of u64
    #[repr(C)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    struct Padded {
        a: u8,
        // implicit padding here before `b`
        b: u32,
    }

    const _: () = {
        assert!(size_of::<Padded>() == size_of::<u64>());
        assert!(align_of::<Padded>() >= align_of::<u32>());
    };

    impl SafeMemory for Padded {}

    unsafe impl Pod for Padded {
        fn is_valid_bitpattern(bytes: &[u8]) -> bool {
            if bytes.len() != 8 {
                return false;
            }

            // Check the real fields
            // a == anything
            // b == anything

            // But padding (bytes[1..4]) must be zero
            bytes[1] == 0 && bytes[2] == 0 && bytes[3] == 0
        }
    }

    let value = Padded {
        a: 0xAA,
        b: 0x11223344,
    };
    let mut zeroed = core::mem::MaybeUninit::<Padded>::zeroed();
    unsafe {
        core::ptr::write(zeroed.as_mut_ptr(), value);
    }
    let binding = unsafe { zeroed.assume_init() };
    let bytes = to_bytes(&binding);

    const BUF_SIZE: usize = size_of::<Padded>();
    let mut buf = [0u8; BUF_SIZE];

    for i in 0..BUF_SIZE {
        // copy the original bytes into the buffer
        buf.copy_from_slice(bytes);

        // flip a bit in one byte
        buf[i] ^= 0xFF;

        let result = from_bytes::<Padded>(&buf);
        let flipped_padding = i > 0 && i < BUF_SIZE - size_of::<u32>(); // flipped in padding bytes

        if flipped_padding {
            assert!(
                result.is_err(),
                "mutation at byte {i} should be rejected due to padding corruption"
            );
        } else {
            assert!(
                result.is_ok(),
                "mutation at byte {i} should be accepted (not padding)"
            );
        }
    }
}
