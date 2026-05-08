use super::*;

pub trait ToPrimFloat {
    fn to_float<T>(&self) -> Result<T>
    where
        T: Float,
        T: Prim;
}

impl<A> ToPrimFloat for A
where
    A: ToPrim {
    fn to_float<T>(&self) -> Result<T>
    where
        T: Float,
        T: Prim {
        match T::VARIANT {
            Variant::F32 => {
                let n: f32 = self.to_f32()?;
                unsafe {
                    Ok(::core::mem::transmute_copy(&n))
                }
            },
            Variant::F64 => {
                let n: f64 = self.to_f64()?;
                unsafe {
                    Ok(::core::mem::transmute_copy(&n))
                }
            },
            _ => {
                unsafe {
                    ::core::hint::unreachable_unchecked()
                }
            }
        }
    }
}
