macro_rules! r#impl {
    ($($ty:ident)*) => {
        $(
            impl CheckedAdd for $ty {
                #[inline]
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    self.checked_add(rhs)
                }
            }

            impl CheckedSub for $ty {
                #[inline]
                fn checked_sub(self, rhs: Self) -> Option<Self> {
                    self.checked_sub(rhs)
                }
            }

            impl CheckedMul for $ty {
                #[inline]
                fn checked_mul(self, rhs: Self) -> Option<Self> {
                    self.checked_mul(rhs)
                }
            }

            impl CheckedDiv for $ty {
                #[inline]
                fn checked_div(self, rhs: Self) -> Option<Self> {
                    self.checked_div(rhs)
                }
            }

            impl CheckedRem for $ty {
                #[inline]
                fn checked_rem(self, rhs: Self) -> Option<Self> {
                    self.checked_rem(rhs)
                }
            }

            impl CheckedShl for $ty {
                #[inline]
                fn checked_shl(self, rhs: u32) -> Option<Self> {
                    self.checked_shl(rhs)
                }
            }

            impl CheckedShr for $ty {
                #[inline]
                fn checked_shr(self, rhs: u32) -> Option<Self> {
                    self.checked_shr(rhs)
                }
            }

            impl CheckedNeg for $ty {
                #[inline]
                fn checked_neg(self) -> Option<Self> {
                    self.checked_neg()
                }
            }
        )*
    };
}

pub trait CheckedAdd
where
    Self: Sized {
    fn checked_add(self, rhs: Self) -> Option<Self>;
}

pub trait CheckedSub
where
    Self: Sized {
    fn checked_sub(self, rhs: Self) -> Option<Self>;
}

pub trait CheckedMul
where
    Self: Sized {
    fn checked_mul(self, rhs: Self) -> Option<Self>;
}

pub trait CheckedDiv
where
    Self: Sized {
    fn checked_div(self, rhs: Self) -> Option<Self>;
}

pub trait CheckedRem
where
    Self: Sized {
    fn checked_rem(self, rhs: Self) -> Option<Self>;
}

pub trait CheckedShl
where
    Self: Sized {
    fn checked_shl(self, rhs: u32) -> Option<Self>;
}

pub trait CheckedShr
where
    Self: Sized {
    fn checked_shr(self, rhs: u32) -> Option<Self>;
}

pub trait CheckedNeg
where
    Self: Sized {
    fn checked_neg(self) -> Option<Self>;
}

r#impl!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);
