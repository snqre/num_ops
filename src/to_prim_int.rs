use super::*;

macro_rules! convert {
    ($T:ident $($self:ident as $var:ident $ty:ty)*) => {
        paste::paste!(
            match T::VARIANT {
                $(
                    Variant::$var => {
                        let n: $ty = $self.[< to_ $ty >]()?;
                        unsafe {
                            return Ok(core::mem::transmute_copy(&n))
                        }
                    },
                )*
                _ => unsafe {
                    core::hint::unreachable_unchecked()
                }
            }
        )
    };
}

pub trait ToPrimInt {
    fn to_int<T>(&self) -> Result<T>
    where
        T: Int,
        T: Prim;
}

impl<A> ToPrimInt for A
where
    A: ToPrim {
    fn to_int<T>(&self) -> Result<T>
    where
        T: Int,
        T: Prim {
        convert!(
            T
            self as U8 u8
            self as U16 u16
            self as U32 u32
            self as U64 u64
            self as U128 u128
            self as USize usize
            self as I8 i8
            self as I16 i16
            self as I32 i32
            self as I64 i64
            self as I128 i128
            self as ISize isize
        )
    }
}
