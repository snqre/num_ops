macro_rules! r#impl {
    ($($ty:ident)*) => {
        $(
            impl OverflowingAdd for $ty {
                #[inline]
                fn overflowing_add(self, rhs: Self) -> Flag<Self> {
                    self.overflowing_add(rhs)
                }
            }

            impl OverflowingSub for $ty {
                #[inline]
                fn overflowing_sub(self, rhs: Self) -> Flag<Self> {
                    self.overflowing_sub(rhs)
                }
            }

            impl OverflowingMul for $ty {
                #[inline]
                fn overflowing_mul(self, rhs: Self) -> Flag<Self> {
                    self.overflowing_mul(rhs)
                }
            }

            impl OverflowingDiv for $ty {
                #[inline]
                fn overflowing_div(self, rhs: Self) -> Flag<Self> {
                    self.overflowing_div(rhs)
                }
            }

            impl OverflowingRem for $ty {
                #[inline]
                fn overflowing_rem(self, rhs: Self) -> Flag<Self> {
                    self.overflowing_rem(rhs)
                }
            }

            impl OverflowingShl for $ty {
                #[inline]
                fn overflowing_shl(self, rhs: u32) -> Flag<Self> {
                    self.overflowing_shl(rhs)
                }
            }

            impl OverflowingShr for $ty {
                #[inline]
                fn overflowing_shr(self, rhs: u32) -> Flag<Self> {
                    self.overflowing_shr(rhs)
                }
            }

            impl OverflowingNeg for $ty {
                #[inline]
                fn overflowing_neg(self) -> Flag<Self> {
                    self.overflowing_neg()
                }
            }
        )*
    };
}

type Flag<T> = (T, bool);

pub trait OverflowingAdd
where
    Self: Sized {
    fn overflowing_add(self, rhs: Self) -> Flag<Self>;
}

pub trait OverflowingSub
where
    Self: Sized {
    fn overflowing_sub(self, rhs: Self) -> Flag<Self>;
}

pub trait OverflowingMul
where
    Self: Sized {
    fn overflowing_mul(self, rhs: Self) -> Flag<Self>;
}

pub trait OverflowingDiv
where
    Self: Sized {
    fn overflowing_div(self, rhs: Self) -> Flag<Self>;
}

pub trait OverflowingRem
where
    Self: Sized {
    fn overflowing_rem(self, rhs: Self) -> Flag<Self>;
}

pub trait OverflowingShl
where
    Self: Sized {
    fn overflowing_shl(self, rhs: u32) -> Flag<Self>;
}

pub trait OverflowingShr
where
    Self: Sized {
    fn overflowing_shr(self, rhs: u32) -> Flag<Self>;
}

pub trait OverflowingNeg
where
    Self: Sized {
    fn overflowing_neg(self) -> Flag<Self>;
}

r#impl!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);
