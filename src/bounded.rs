use std::{
    usize,
    u8,
    u16,
    u32,
    u64,
    u128,
    isize,
    i8,
    i16,
    i32,
    i64,
    i128
};

pub trait Bounded {
    fn min() -> Self;
    fn max() -> Self;
}

impl Bounded for usize {
    fn min() -> usize { usize::MIN }
    fn max() -> usize { usize::MAX }
}

impl Bounded for u8 {
    fn min() -> u8 { u8::MIN }
    fn max() -> u8 { u8::MAX }
}

impl Bounded for u16 {
    fn min() -> u16 { u16::MIN }
    fn max() -> u16 { u16::MAX }
}

impl Bounded for u32 {
    fn min() -> u32 { u32::MIN }
    fn max() -> u32 { u32::MAX }
}

impl Bounded for u64 {
    fn min() -> u64 { u64::MIN }
    fn max() -> u64 { u64::MAX }
}

impl Bounded for u128 {
    fn min() -> u128 { u128::MIN }
    fn max() -> u128 { u128::MAX }
}

impl Bounded for isize {
    fn min() -> isize { isize::MIN }
    fn max() -> isize { isize::MAX }
}

impl Bounded for i8 {
    fn min() -> i8 { i8::MIN }
    fn max() -> i8 { i8::MAX }
}

impl Bounded for i16 {
    fn min() -> i16 { i16::MIN }
    fn max() -> i16 { i16::MAX }
}

impl Bounded for i32 {
    fn min() -> i32 { i32::MIN }
    fn max() -> i32 { i32::MAX }
}

impl Bounded for i64 {
    fn min() -> i64 { i64::MIN }
    fn max() -> i64 { i64::MAX }
}

impl Bounded for i128 {
    fn min() -> i128 { i128::MIN }
    fn max() -> i128 { i128::MAX }
}
