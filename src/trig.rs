macro_rules! r#impl {
    ($($ty:ident)*) => {
        $(
            impl ArcTan2 for $ty {
                #[inline]
                fn atan2(self, rhs: Self) -> Self {
                    self.atan2(rhs)
                }
            }

            impl ArcTan for $ty {
                #[inline]
                fn atan(self) -> Self {
                    self.atan()
                }
            }

            impl ArcSin for $ty {
                #[inline]
                fn asin(self) -> Self {
                    self.asin()
                }
            }

            impl ArcCos for $ty {
                #[inline]
                fn acos(self) -> Self {
                    self.acos()
                }
            }

            impl Tan for $ty {
                #[inline]
                fn tan(self) -> Self {
                    self.tan()
                }
            }

            impl Sin for $ty {
                #[inline]
                fn sin(self) -> Self {
                    self.sin()
                }
            }

            impl Cos for $ty {
                #[inline]
                fn cos(self) -> Self {
                    self.cos()
                }
            }

            impl ToRad for $ty {
                #[inline]
                fn to_rad(self) -> Self {
                    self.to_radians()
                }
            }

            impl ToDeg for $ty {
                #[inline]
                fn to_deg(self) -> Self {
                    self.to_degrees()
                }
            }
        )*
    };
}

pub trait ArcTan2 {
    fn atan2(self, rhs: Self) -> Self;
}

pub trait ArcTan {
    fn atan(self) -> Self;
}

pub trait ArcSin {
    fn asin(self) -> Self;
}

pub trait ArcCos {
    fn acos(self) -> Self;
}

pub trait Tan {
    fn tan(self) -> Self;
}

pub trait Sin {
    fn sin(self) -> Self;
}

pub trait Cos {
    fn cos(self) -> Self;
}

pub trait ToRad {
    fn to_rad(self) -> Self;
}

pub trait ToDeg {
    fn to_deg(self) -> Self;
}

r#impl!(
    f32
    f64
);
