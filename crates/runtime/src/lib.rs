#![cfg_attr(not(feature = "std"), no_std)]

//! Runtime components of Quanto language.

extern crate alloc;

use crate::value::Value;

pub mod global;
pub mod rt;
pub mod value;

pub struct Program<'a, const N: usize> {
    src: &'a str,
}
impl<'a, const N: usize> Program<'a, N> {
    pub const fn parse(src: &'a str) -> Self {
        Self { src }
    }

    pub fn bind(self, args: &'a [Value; N]) -> BoundProgram<'a, N> {
        BoundProgram {
            program: self,
            args,
        }
    }
}

pub struct BoundProgram<'a, const N: usize> {
    program: Program<'a, N>,
    args: &'a [Value; N],
}
