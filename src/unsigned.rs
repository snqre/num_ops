macro_rules! r#impl {
    ($($ty:ident)*) => {
        $(
            impl Unsigned for $ty {}
        )*
    };
}

pub trait Unsigned {}

r#impl!(
    u8
    u16
    u32
    u64
    u128
    usize
);
