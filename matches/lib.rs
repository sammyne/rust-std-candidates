#![no_std]

extern crate sgx_tstd as std;

/// Check if an expression matches a refutable pattern.
///
/// Syntax: `matches!(` *expression* `,` *pattern* `)`
///
/// Return a boolean, true if the expression matches the pattern, false otherwise.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate matches;
///
/// pub enum Foo<T> {
///     A,
///     B(T),
/// }
///
/// impl<T> Foo<T> {
///     pub fn is_a(&self) -> bool {
///         matches!(*self, Foo::A)
///     }
///
///     pub fn is_b(&self) -> bool {
///         matches!(*self, Foo::B(_))
///     }
/// }
///
/// # fn main() { }
/// ```
#[macro_export]
macro_rules! matches {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => true,
            _ => false
        }
    }
}

/// Assert that an expression matches a refutable pattern.
///
/// Syntax: `assert_matches!(` *expression* `,` *pattern* `)`
///
/// Panic with a message that shows the expression if it does not match the
/// pattern.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate matches;
///
/// fn main() {
///     let data = [1, 2, 3];
///     assert_matches!(data.get(1), Some(_));
/// }
/// ```
#[macro_export]
macro_rules! assert_matches {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => (),
            ref e => panic!("assertion failed: `{:?}` does not match `{}`", e, stringify!($($pattern)+)),
        }
    }
}

/// Assert that an expression matches a refutable pattern using debug assertions.
///
/// Syntax: `debug_assert_matches!(` *expression* `,` *pattern* `)`
///
/// If debug assertions are enabled, panic with a message that shows the
/// expression if it does not match the pattern.
///
/// When debug assertions are not enabled, this macro does nothing.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate matches;
///
/// fn main() {
///     let data = [1, 2, 3];
///     debug_assert_matches!(data.get(1), Some(_));
/// }
/// ```
#[macro_export]
macro_rules! debug_assert_matches {
    ($expression:expr, $($pattern:tt)+) => {
        if cfg!(debug_assertions) {
            match $expression {
                $($pattern)+ => (),
                ref e => panic!("assertion failed: `{:?}` does not match `{}`", e, stringify!($($pattern)+)),
            }
        }
    }
}

//#[test]
#[cfg(feature = "testing")]
fn matches_works() {
    let foo = Some("-12");
    assert!(matches!(foo, Some(bar) if
        matches!(bar.as_bytes()[0], b'+' | b'-') &&
        matches!(bar.as_bytes()[1], b'0'...b'9')
    ));
}

//#[test]
#[cfg(feature = "testing")]
fn assert_matches_works() {
    let foo = Some("-12");
    assert_matches!(foo, Some(bar) if
        matches!(bar.as_bytes()[0], b'+' | b'-') &&
        matches!(bar.as_bytes()[1], b'0'...b'9')
    );
}

//#[test]
//#[should_panic(expected = "assertion failed: `Some(\"-AB\")` does not match ")]
#[cfg(feature = "testing")]
fn assert_matches_panics() {
    let foo = Some("-AB");
    assert_matches!(foo, Some(bar) if
        matches!(bar.as_bytes()[0], b'+' | b'-') &&
        matches!(bar.as_bytes()[1], b'0'...b'9')
    );
}

#[cfg(feature = "testing")]
mod sgx_tests;

#[cfg(feature = "testing")]
pub use sgx_tests::do_rsgx_test;
