#![no_std]

modwire::expose!(
    pub bits
    pub bound
    pub checked
    pub checked2
    pub from_prim
    pub overflowing
    pub pow
    pub prim
    pub quick_access
    pub root
    pub sign
    pub signed
    pub to_prim_float
    pub to_prim_int
    pub to_prim
    pub trig
    pub unsigned
    pub wrapping
);

pub type Result<T> = core::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(Hash)]
#[derive(strum::EnumCount)]
#[derive(strum::EnumIs)]
#[derive(thiserror::Error)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub enum Error {
    #[error("{}", 0)]
    Overflow,
    #[error("{}", 0)]
    Underflow,
    #[error("{}", 0)]
    DivisionByZero,
    #[error("{}", 0)]
    ModuloByZero,
    #[error("{}", 0)]
    ShiftOverflow,
    #[error("{}", 0)]
    NegationOverflow,
    #[error("{}", 0)]
    UnsupportedConversion
}

pub trait Int
where
    Self: core::marker::Sized,
    Self: core::cmp::Eq,
    Self: core::cmp::Ord,
    Self: Num,
    Self: Bits,
    Self: Bound,
    Self: Sign,
    Self: CheckedAdd,
    Self: CheckedDiv,
    Self: CheckedMul,
    Self: CheckedNeg,
    Self: CheckedRem,
    Self: CheckedShl,
    Self: CheckedShr,
    Self: CheckedSub,
    Self: OverflowingAdd,
    Self: OverflowingDiv,
    Self: OverflowingMul,
    Self: OverflowingNeg,
    Self: OverflowingRem,
    Self: OverflowingShl,
    Self: OverflowingShr,
    Self: OverflowingSub,
    Self: WrappingAdd,
    Self: WrappingDiv,
    Self: WrappingMul,
    Self: WrappingNeg,
    Self: WrappingPow,
    Self: WrappingRem,
    Self: WrappingSub,
    Self: core::ops::Shl<Output = Self>,
    Self: core::ops::ShlAssign,
    Self: core::ops::Shr<Output = Self>,
    Self: core::ops::ShrAssign,
    Self: core::ops::BitAnd<Output = Self>,
    Self: core::ops::BitAndAssign,
    Self: core::ops::BitOr<Output = Self>,
    Self: core::ops::BitOrAssign,
    Self: core::ops::BitXor<Output = Self>,
    Self: core::ops::BitXorAssign,
    Self: for<'a> core::ops::Shl<&'a Self, Output = Self>,
    Self: for<'a> core::ops::ShlAssign<&'a Self>,
    Self: for<'a> core::ops::Shr<&'a Self, Output = Self>,
    Self: for<'a> core::ops::ShrAssign<&'a Self>,
    Self: for<'a> core::ops::BitAnd<&'a Self, Output = Self>,
    Self: for<'a> core::ops::BitAndAssign<&'a Self>,
    Self: for<'a> core::ops::BitOr<&'a Self, Output = Self>,
    Self: for<'a> core::ops::BitOrAssign<&'a Self>,
    Self: for<'a> core::ops::BitXor<&'a Self, Output = Self>,
    Self: for<'a> core::ops::BitXorAssign<&'a Self>,
    Self: core::convert::TryFrom<i8>,
    Self: core::convert::TryFrom<i16>,
    Self: core::convert::TryFrom<i32>,
    Self: core::convert::TryFrom<i64>,
    Self: core::convert::TryFrom<i128>,
    Self: core::convert::TryFrom<isize>,
    Self: core::convert::TryFrom<u8>,
    Self: core::convert::TryFrom<u16>,
    Self: core::convert::TryFrom<u32>,
    Self: core::convert::TryFrom<u64>,
    Self: core::convert::TryFrom<u128>,
    Self: core::convert::TryFrom<usize>,
    Self: core::convert::TryInto<i8>,
    Self: core::convert::TryInto<i16>,
    Self: core::convert::TryInto<i32>,
    Self: core::convert::TryInto<i64>,
    Self: core::convert::TryInto<i128>,
    Self: core::convert::TryInto<isize>,
    Self: core::convert::TryInto<u8>,
    Self: core::convert::TryInto<u16>,
    Self: core::convert::TryInto<u32>,
    Self: core::convert::TryInto<u64>,
    Self: core::convert::TryInto<u128>,
    Self: core::convert::TryInto<usize> {}

impl<T> Int for T
where
    T: core::marker::Sized,
    T: core::cmp::Eq,
    T: core::cmp::Ord,
    T: Num,
    T: Bits,
    T: Bound,
    T: Sign,
    T: CheckedAdd,
    T: CheckedDiv,
    T: CheckedMul,
    T: CheckedNeg,
    T: CheckedRem,
    T: CheckedShl,
    T: CheckedShr,
    T: CheckedSub,
    T: OverflowingAdd,
    T: OverflowingDiv,
    T: OverflowingMul,
    T: OverflowingNeg,
    T: OverflowingRem,
    T: OverflowingShl,
    T: OverflowingShr,
    T: OverflowingSub,
    T: WrappingAdd,
    T: WrappingDiv,
    T: WrappingMul,
    T: WrappingNeg,
    T: WrappingPow,
    T: WrappingRem,
    T: WrappingSub,
    T: core::ops::Shl<Output = Self>,
    T: core::ops::ShlAssign,
    T: core::ops::Shr<Output = Self>,
    T: core::ops::ShrAssign,
    T: core::ops::BitAnd<Output = Self>,
    T: core::ops::BitAndAssign,
    T: core::ops::BitOr<Output = Self>,
    T: core::ops::BitOrAssign,
    T: core::ops::BitXor<Output = Self>,
    T: core::ops::BitXorAssign,
    T: for<'a> core::ops::Shl<&'a Self, Output = Self>,
    T: for<'a> core::ops::ShlAssign<&'a Self>,
    T: for<'a> core::ops::Shr<&'a Self, Output = Self>,
    T: for<'a> core::ops::ShrAssign<&'a Self>,
    T: for<'a> core::ops::BitAnd<&'a Self, Output = Self>,
    T: for<'a> core::ops::BitAndAssign<&'a Self>,
    T: for<'a> core::ops::BitOr<&'a Self, Output = Self>,
    T: for<'a> core::ops::BitOrAssign<&'a Self>,
    T: for<'a> core::ops::BitXor<&'a Self, Output = Self>,
    T: for<'a> core::ops::BitXorAssign<&'a Self>,
    T: core::convert::TryFrom<i8>,
    T: core::convert::TryFrom<i16>,
    T: core::convert::TryFrom<i32>,
    T: core::convert::TryFrom<i64>,
    T: core::convert::TryFrom<i128>,
    T: core::convert::TryFrom<isize>,
    T: core::convert::TryFrom<u8>,
    T: core::convert::TryFrom<u16>,
    T: core::convert::TryFrom<u32>,
    T: core::convert::TryFrom<u64>,
    T: core::convert::TryFrom<u128>,
    T: core::convert::TryFrom<usize>,
    T: core::convert::TryInto<i8>,
    T: core::convert::TryInto<i16>,
    T: core::convert::TryInto<i32>,
    T: core::convert::TryInto<i64>,
    T: core::convert::TryInto<i128>,
    T: core::convert::TryInto<isize>,
    T: core::convert::TryInto<u8>,
    T: core::convert::TryInto<u16>,
    T: core::convert::TryInto<u32>,
    T: core::convert::TryInto<u64>,
    T: core::convert::TryInto<u128>,
    T: core::convert::TryInto<usize> {}

pub trait Float
where
    Self: core::marker::Sized,
    Self: Num,
    Self: ArcTan,
    Self: ArcSin,
    Self: ArcCos,
    Self: Tan,
    Self: Sin,
    Self: Cos,
    Self: ToRad,
    Self: ToDeg,
    Self: PowF,
    Self: PowI {}

impl<T> Float for T
where
    T: core::marker::Sized,
    T: Num,
    T: QuickAccess,
    T: ArcTan,
    T: ArcSin,
    T: ArcCos,
    T: Tan,
    T: Sin,
    T: Cos,
    T: ToRad,
    T: ToDeg,
    T: PowF,
    T: PowI {}

pub trait Num
where
    Self: QuickAccess,
    Self: Sqrt,
    Self: ToPrim,
    Self: core::default::Default,
    Self: core::clone::Clone,
    Self: core::marker::Sized,
    Self: core::marker::Copy,
    Self: core::cmp::PartialEq,
    Self: core::cmp::PartialOrd,
    Self: core::fmt::Debug,
    Self: core::fmt::Display,
    Self: core::ops::Add<Output = Self>,
    Self: core::ops::AddAssign,
    Self: core::ops::Sub<Output = Self>,
    Self: core::ops::SubAssign,
    Self: core::ops::Mul<Output = Self>,
    Self: core::ops::MulAssign,
    Self: core::ops::Div<Output = Self>,
    Self: core::ops::DivAssign,
    Self: core::ops::Rem<Output = Self>,
    Self: core::ops::RemAssign,
    Self: for<'a> core::ops::Add<&'a Self, Output = Self>,
    Self: for<'a> core::ops::AddAssign<&'a Self>,
    Self: for<'a> core::ops::Sub<&'a Self, Output = Self>,
    Self: for<'a> core::ops::SubAssign<&'a Self>,
    Self: for<'a> core::ops::Mul<&'a Self, Output = Self>,
    Self: for<'a> core::ops::MulAssign<&'a Self>,
    Self: for<'a> core::ops::Div<&'a Self, Output = Self>,
    Self: for<'a> core::ops::DivAssign<&'a Self>,
    Self: for<'a> core::ops::Rem<&'a Self, Output = Self>,
    Self: for<'a> core::ops::RemAssign<&'a Self> {}

impl<T> Num for T
where
    T: QuickAccess,
    T: Sqrt,
    T: ToPrim,
    T: core::default::Default,
    T: core::clone::Clone,
    T: core::marker::Sized,
    T: core::marker::Copy,
    T: core::cmp::PartialEq,
    T: core::cmp::PartialOrd,
    T: core::fmt::Debug,
    T: core::fmt::Display,
    T: core::ops::Add<Output = Self>,
    T: core::ops::AddAssign,
    T: core::ops::Sub<Output = Self>,
    T: core::ops::SubAssign,
    T: core::ops::Mul<Output = Self>,
    T: core::ops::MulAssign,
    T: core::ops::Div<Output = Self>,
    T: core::ops::DivAssign,
    T: core::ops::Rem<Output = Self>,
    T: core::ops::RemAssign,
    T: for<'a> core::ops::Add<&'a Self, Output = Self>,
    T: for<'a> core::ops::AddAssign<&'a Self>,
    T: for<'a> core::ops::Sub<&'a Self, Output = Self>,
    T: for<'a> core::ops::SubAssign<&'a Self>,
    T: for<'a> core::ops::Mul<&'a Self, Output = Self>,
    T: for<'a> core::ops::MulAssign<&'a Self>,
    T: for<'a> core::ops::Div<&'a Self, Output = Self>,
    T: for<'a> core::ops::DivAssign<&'a Self>,
    T: for<'a> core::ops::Rem<&'a Self, Output = Self>,
    T: for<'a> core::ops::RemAssign<&'a Self> {}
