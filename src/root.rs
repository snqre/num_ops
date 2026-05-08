macro_rules! float_impl {
    ($($ty:ident)*) => {
        $(
            impl Sqrt for $ty {
                #[inline]
                fn sqrt(self) -> Self {
                    self.sqrt()
                }
            }

            impl Cbrt for $ty {
                #[inline]
                fn cbrt(self) -> Self {
                    self.cbrt()
                }
            }
        )*
    };
}

macro_rules! int_impl {
    ($($ty:ident)*) => {
        $(
            impl Sqrt for $ty {
                fn sqrt(self) -> Self {
                    self.isqrt()
                }
            }
        )*
    };
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

pub trait Cbrt {
    fn cbrt(self) -> Self;
}

float_impl!(
    f32
    f64
);

int_impl!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);
