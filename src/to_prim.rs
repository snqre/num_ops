use super::*;

macro_rules! int_conversion_fn {
    ($($ty:ty)*) => {
        paste::paste!(
            $(
                fn [< to _ $ty >](&self) -> Result<$ty> {
                    let ret: Self = *self;
                    let ret: $ty = $ty::try_from(ret).ok().ok_or(Error::UnsupportedConversion)?;
                    Ok(ret)
                }
            )*
        );
    };
}

macro_rules! impl_int {
    ($($ty:ty)*) => {
        $(
            impl ToPrim for $ty {
                int_conversion_fn!(
                    u8 u16 u32 u64 u128 usize
                    i8 i16 i32 i64 i128 isize
                );

                fn to_f32(&self) -> Result<f32> {
                    let ret: Self = *self;
                    let ret: f32 = ret as f32;
                    Ok(ret)
                }

                fn to_f64(&self) -> Result<f64> {
                    let ret: Self = *self;
                    let ret: f64 = ret as f64;
                    Ok(ret)
                }
            }
        )*
    };
}

macro_rules! float_conversion_fn {
    ($($ty:ty)*) => {
        paste::paste!(
            $(
                fn [< to _ $ty >](&self) -> Result<$ty> {
                    let ret: Self = *self;
                    if !ret.is_finite() {
                        return Err(Error::UnsupportedConversion)
                    }
                    if ret > $ty::MAX as Self {
                        return Err(Error::Overflow)
                    }
                    if ret < $ty::MIN as Self {
                        return Err(Error::Underflow)
                    }
                    Ok(ret as $ty)
                }
            )*
        );
    };
}

macro_rules! impl_float {
    ($($ty:ty)*) => {
        $(
            impl ToPrim for $ty {
                float_conversion_fn!(
                    u8 u16 u32 u64 u128 usize
                    i8 i16 i32 i64 i128 isize
                );

                fn to_f32(&self) -> Result<f32> {
                    let ret: Self = *self;
                    let ret: f32 = ret as f32;
                    Ok(ret)
                }

                fn to_f64(&self) -> Result<f64> {
                    let ret: Self = *self;
                    let ret: f64 = ret as f64;
                    Ok(ret)
                }
            }
        )*
    };
}

pub trait ToPrim {
    fn to_u8(&self) -> Result<u8>;
    fn to_u16(&self) -> Result<u16>;
    fn to_u32(&self) -> Result<u32>;
    fn to_u64(&self) -> Result<u64>;
    fn to_u128(&self) -> Result<u128>;
    fn to_usize(&self) -> Result<usize>;
    fn to_i8(&self) -> Result<i8>;
    fn to_i16(&self) -> Result<i16>;
    fn to_i32(&self) -> Result<i32>;
    fn to_i64(&self) -> Result<i64>;
    fn to_i128(&self) -> Result<i128>;
    fn to_isize(&self) -> Result<isize>;
    fn to_f32(&self) -> Result<f32>;
    fn to_f64(&self) -> Result<f64>;
}

impl_int!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);

impl_float!(
    f32 f64
);
