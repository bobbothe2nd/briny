//! Checks for asserting no undefined behavior occurs.

/// Aborts the program if the conditions are not met.
///
/// These should not be relied on for safety, and are only inserted in code
/// at certain times.
#[macro_export]
macro_rules! ub_assert {
    ($($(($($def:ident = $val:expr,)*) => $result:block)+, $msg:literal)+) => {
        #[cfg(debug_assertions)]
        $(
            $(
                if !({
                    $( let $def = $val; )*
                    $result
                }) {
                    $crate::ub::abort($msg);
                }
            )+
        )+
    };
}

/// Aborts the program by recursively panicking during unwind.
///
/// # Panics
///
/// This will panic indefinitely - until the program forces it to abort.
pub const fn abort(msg: &str) -> ! {
    pub struct PanicOnDrop;

    impl Drop for PanicOnDrop {
        #[allow(unconditional_recursion)] // thats the point
        fn drop(&mut self) {
            // make new self to drop
            let _no_unwind = Self;
            // panic to abort unwind
            panic!("program unwinding");
        }
    }

    loop {
        let _abort = PanicOnDrop;
        panic!("{}", msg); // cause panic while `PanicOnDrop` is live
    }
}
