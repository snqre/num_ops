macro_rules! signed_impl {
    ($($ty:ident)*) => {
        $(
            impl Sign for $ty {
                const SIGNED: bool = true;
            }
        )*
    };
}

macro_rules! unsigned_impl {
    ($($ty:ident)*) => {
        $(
            impl Sign for $ty {
                const SIGNED: bool = false;
            }
        )*
    };
}

pub trait Sign {
    const SIGNED: bool;
}

unsigned_impl!(
    u8
    u16
    u32
    u64
    u128
    usize
);

signed_impl!(
    i8
    i16
    i32
    i64
    i128
    isize
);
