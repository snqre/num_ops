use super::*;

macro_rules! r#impl {
    ($($ty:ty)*) => {
        paste::paste!(
            $(
                impl FromPrim for $ty {
                    #[inline]
                    fn from<T>(n: T) -> Result<Self>
                    where
                        T: Prim,
                        T: ToPrim {
                        match T::VARIANT {
                            Variant::U8 => n.[< to_ $ty >](),
                            Variant::U16 => n.[< to_ $ty >](),
                            Variant::U32 => n.[< to_ $ty >](),
                            Variant::U64 => n.[< to_ $ty >](),
                            Variant::U128 => n.[< to_ $ty >](),
                            Variant::USize => n.[< to_ $ty >](),
                            Variant::I8 => n.[< to_ $ty >](),
                            Variant::I16 => n.[< to_ $ty >](),
                            Variant::I32 => n.[< to_ $ty >](),
                            Variant::I64 => n.[< to_ $ty >](),
                            Variant::I128 => n.[< to_ $ty >](),
                            Variant::ISize => n.[< to_ $ty >](),
                            Variant::F32 => n.[< to_ $ty >](),
                            Variant::F64 => n.[< to_ $ty >]()
                        }
                    }
                }
            )*
        );
    };
}

pub trait FromPrim
where
    Self: Sized {
    fn from<T>(n: T) -> Result<Self>
    where
        T: Prim,
        T: ToPrim;
}

r#impl!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
);
