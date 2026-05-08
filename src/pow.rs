macro_rules! r#impl {
    ($($ty:ident)*) => {
        $(
            impl PowF for $ty {
                #[inline]
                fn powf(self, exp: Self) -> Self {
                    self.powf(exp)
                }
            }

            impl PowI for $ty {
                #[inline]
                fn powi(self, exp: i32) -> Self {
                    self.powi(exp)
                }
            }
        )*
    };
}

pub trait PowF {
    fn powf(self, exp: Self) -> Self;
}

pub trait PowI {
    fn powi(self, exp: i32) -> Self;
}

r#impl!(
    f32
    f64
);
