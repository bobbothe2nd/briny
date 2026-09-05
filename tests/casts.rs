use briny::raw::cast::{cast, from_bytes, to_bytes};
use briny::traits::{Pod, RawConvert, StableLayout};

#[test]
fn to_bytes_roundtrip_fuzz() {
    #[repr(packed)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    struct Pair {
        a: u32,
        b: u64,
    }

    unsafe impl StableLayout for Pair {}
    unsafe impl RawConvert for Pair {}
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
fn cast_struct_edge_fuzz() {
    #[repr(C, align(4))]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
    struct FourBytes {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
    }

    unsafe impl StableLayout for FourBytes {}
    unsafe impl RawConvert for FourBytes {}
    unsafe impl Pod for FourBytes {}

    const _: () = {
        assert!(size_of::<FourBytes>() == size_of::<u32>());
        assert!(align_of::<FourBytes>() == align_of::<u32>());
    };

    for a in 0u8..=10 {
        for b in 0u8..=10 {
            for c in 0u8..=10 {
                for d in 0u8..=10 {
                    let orig = FourBytes { a, b, c, d };
                    let casted: u32 = cast(&orig);
                    let back: FourBytes = cast(&casted);
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

    unsafe impl RawConvert for A {}
    unsafe impl StableLayout for A {}
    unsafe impl Pod for A {}

    #[repr(C, align(4))]
    #[derive(Copy, Clone, Debug, PartialEq, Default)]
    struct B(u32);

    unsafe impl RawConvert for B {}
    unsafe impl StableLayout for B {}
    unsafe impl Pod for B {}

    const _: () = {
        assert!(size_of::<A>() == size_of::<B>());
        assert!(align_of::<A>() >= align_of::<B>());
    };

    for v in 0u32..100 {
        let b = B(v);
        let a = cast::<B, A>(&b);
        let b2 = cast::<A, B>(&a);
        assert_eq!(b, b2, "roundtrip failed for value {v}");
    }
}
