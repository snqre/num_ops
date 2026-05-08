macro_rules! r#impl {
    ($($ty:ident)*) => {
        $(
            impl Bound for $ty {
                const MAX: Self = $ty::MAX;
                const MAX_AS_U128: u128 = Self::MAX as u128;
                const MAX_AS_I128: i128 = Self::MIN as i128;
                const MIN: Self = $ty::MIN;
                const MIN_AS_U128: u128 = Self::MIN as u128;
                const MIN_AS_I128: i128 = Self::MIN as i128;
            }
        )*
    };
}

pub trait Bound {
    const MAX: Self;
    const MAX_AS_U128: u128;
    const MAX_AS_I128: i128;
    const MIN: Self;
    const MIN_AS_U128: u128 = 0;
    const MIN_AS_I128: i128 = 0;
}

r#impl!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);
