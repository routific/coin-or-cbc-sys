//! # Coin-OR Cbc library bindings
//!
//! This crate provides bindings to the [Coin-OR] Branch-and-Cut ([CBC]) library. The bindings are
//! generated using [bindgen]. CBC is implemented in C++ but those bindings maps to the provided [C
//! interface] of CBC.
//!
//! Usage of this crate creates runtime dependency on the CBC C++ library so be sure to install
//! that (for example using [coinbrew]) and a suitable LP solver.
//!
//! [Coin-OR]: https://www.coin-or.org
//! [CBC]: https://github.com/coin-or/Cbc
//! [bindgen]: https://crates.io/crates/bindgen
//! [C interface]: https://github.com/coin-or/Cbc/blob/master/Cbc/src/Cbc_C_Interface.h
//! [coinbrew]: https://github.com/coin-or/coinbrew

#[allow(non_camel_case_types, unused_imports, non_upper_case_globals)]
mod ffi;

pub use crate::ffi::*;
