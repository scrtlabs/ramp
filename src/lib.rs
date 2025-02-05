// Copyright 2015 The Ramp Developers
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

//! # Deprecated
//!
//! **This crate is no longer being maintained.**
//!
//! Fortunately, there's [lots of alternatives](https://crates.io/keywords/bignum) to `ramp`.
//! Thanks for the good times :`)

#![crate_type = "lib"]
#![crate_name = "ramp"]
#![feature(core_intrinsics, allocator_api)]
#![feature(step_trait, ptr_internals, try_reserve_kind, raw_vec_internals)]
#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

extern crate alloc;
extern crate core;
extern crate hamming;
#[cfg(feature = "floats")]
extern crate ieee754;
extern crate num_integer;
extern crate num_traits;
extern crate rand;

pub mod ll;
mod mem;
mod raw_vec;

pub mod int;
#[cfg(feature = "floats")]
mod rational;
pub mod traits;

// Re-exports

pub use int::Int;
pub use int::RandomInt;
