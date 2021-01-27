#![allow(clippy::many_single_char_names)]
#![allow(clippy::new_without_default)]
#![allow(clippy::suspicious_op_assign_impl)]

pub mod alg;
pub mod assign;
pub mod bit;
pub mod bool;
pub mod cast;
pub mod cmp;
pub mod conv;
pub mod dfa;
pub mod ds;
pub mod float;
pub mod fp;
pub mod func;
pub mod fxhash;
pub mod graph;
pub mod hash;
pub mod int;
pub mod io;
pub mod io_interactive;
pub mod iter;
pub mod make_vec;
pub mod math;
pub mod mint;
pub mod num;
pub mod rand;
pub mod slice;
pub mod u64;
pub mod vec;
pub mod zo;

pub mod tests;

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! dbg {
	($($x:expr),*) => { std::dbg!($($x),*) }
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! dbg {
	($($x:expr),*) => { std::convert::identity(($($x),*)) }
}
