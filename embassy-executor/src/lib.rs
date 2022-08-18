#![cfg_attr(not(any(feature = "std", feature = "wasm")), no_std)]
#![cfg_attr(feature = "nightly", feature(generic_associated_types, type_alias_impl_trait))]
#![cfg_attr(all(feature = "nightly", target_arch = "xtensa"), feature(asm_experimental_arch))]
#![cfg_attr(feature = "rtos-trace", feature(const_type_name))]
#![allow(clippy::new_without_default)]
#![doc = include_str!("../../README.md")]
#![warn(missing_docs)]

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

pub mod executor;
#[cfg(feature = "time")]
pub mod time;

#[cfg(feature = "nightly")]
pub use embassy_macros::{main, task};

#[doc(hidden)]
/// Implementation details for embassy macros. DO NOT USE.
pub mod export {
    pub use atomic_polyfill as atomic;
    #[cfg(feature = "rtos-trace")]
    pub use rtos_trace::trace;

    /// Expands the given block of code when `embassy-executor` is compiled with
    /// the `rtos-trace-interrupt` feature.
    #[doc(hidden)]
    #[macro_export]
    #[cfg(feature = "rtos-trace-interrupt")]
    macro_rules! rtos_trace_interrupt {
        ($($tt:tt)*) => { $($tt)* };
    }

    /// Does not expand the given block of code when `embassy-executor` is
    /// compiled without the `rtos-trace-interrupt` feature.
    #[doc(hidden)]
    #[macro_export]
    #[cfg(not(feature = "rtos-trace-interrupt"))]
    macro_rules! rtos_trace_interrupt {
        ($($tt:tt)*) => {};
    }
}
