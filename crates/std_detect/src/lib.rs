//! Run-time feature detection for the Rust standard library.
//!
//! To detect whether a feature is enabled in the system running the binary
//! use one of the appropriate macro for the target:
//!
//! * `x86` and `x86_64`: [`is_x86_feature_detected`]
//! * `arm`: [`is_arm_feature_detected`]
//! * `aarch64`: [`is_aarch64_feature_detected`]
//! * `riscv`: [`is_riscv_feature_detected`]
//! * `mips`: [`is_mips_feature_detected`]
//! * `mips64`: [`is_mips64_feature_detected`]
//! * `powerpc`: [`is_powerpc_feature_detected`]
//! * `powerpc64`: [`is_powerpc64_feature_detected`]

#![stable(feature = "stdsimd", since = "1.27.0")]
#![feature(staged_api, doc_cfg, allow_internal_unstable)]
#![deny(rust_2018_idioms)]
#![allow(clippy::shadow_reuse)]
#![cfg_attr(test, allow(unused_imports))]
#![no_std]
#![allow(internal_features)]
// Temporary hack: needed to build against toolchains from before the mass feature renaming.
// Remove this as soon as the stdarch submodule is updated on nightly.
#![allow(stable_features)]
#![feature(stdsimd)]

#[cfg(test)]
#[macro_use]
extern crate std;

// rust-lang/rust#83888: removing `extern crate` gives an error that `vec_spare>
#[cfg_attr(feature = "std_detect_file_io", allow(unused_extern_crates))]
#[cfg(feature = "std_detect_file_io")]
extern crate alloc;

#[doc(hidden)]
#[stable(feature = "stdsimd", since = "1.27.0")]
pub mod detect;
