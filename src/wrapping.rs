macro_rules! r#impl {
    ($($ty:ident)*) => {
        $(
            impl WrappingAdd for $ty {
                #[inline]
                fn wrapping_add(self, rhs: Self) -> Self {
                    self.wrapping_add(rhs)
                }
            }

            impl WrappingSub for $ty {
                #[inline]
                fn wrapping_sub(self, rhs: Self) -> Self {
                    self.wrapping_sub(rhs)
                }
            }

            impl WrappingMul for $ty {
                #[inline]
                fn wrapping_mul(self, rhs: Self) -> Self {
                    self.wrapping_mul(rhs)
                }
            }

            impl WrappingDiv for $ty {
                #[inline]
                fn wrapping_div(self, rhs: Self) -> Self {
                    self.wrapping_div(rhs)
                }
            }

            impl WrappingRem for $ty {
                #[inline]
                fn wrapping_rem(self, rhs: Self) -> Self {
                    self.wrapping_rem(rhs)
                }
            }

            impl WrappingNeg for $ty {
                #[inline]
                fn wrapping_neg(self) -> Self {
                    self.wrapping_neg()
                }
            }

            impl WrappingPow for $ty {
                #[inline]
                fn wrapping_pow(self, exp: u32) -> Self {
                    self.wrapping_pow(exp)
                }
            }
        )*
    };
}

pub trait WrappingAdd
where
    Self: Sized {
    fn wrapping_add(self, rhs: Self) -> Self;
}

pub trait WrappingSub
where
    Self: Sized {
    fn wrapping_sub(self, rhs: Self) -> Self;
}

pub trait WrappingMul
where
    Self: Sized {
    fn wrapping_mul(self, rhs: Self) -> Self;
}

pub trait WrappingDiv
where
    Self: Sized {
    fn wrapping_div(self, rhs: Self) -> Self;
}

pub trait WrappingRem
where
    Self: Sized {
    fn wrapping_rem(self, rhs: Self) -> Self;
}

pub trait WrappingNeg
where
    Self: Sized {
    fn wrapping_neg(self) -> Self;
}

pub trait WrappingPow
where
    Self: Sized {
    fn wrapping_pow(self, exp: u32) -> Self;
}

r#impl!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);
