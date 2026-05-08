macro_rules! r#impl {
    ($($ty:ident)*) => {
        $(
            impl Bits for $ty {
                const BITS: Self = $ty::BITS as Self;
                const BITS_AS_U128: u128 = Self::BITS as u128;
                const BITS_AS_I128: i128 = Self::BITS as i128;
            }
        )*
    };
}

pub trait Bits {
    const BITS: Self;
    const BITS_AS_U128: u128;
    const BITS_AS_I128: i128;
}

r#impl!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);
