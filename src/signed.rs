macro_rules! r#impl {
    ($($ty:ident)*) => {
        $(
            impl Signed for $ty {}
        )*
    };
}

pub trait Signed {}

r#impl!(
    i8
    i16
    i32
    i64
    i128
    isize
    f32
    f64
);
