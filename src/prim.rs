macro_rules! variant {
    ($($ty:ty as $ident:ident)*) => {
        $(
            impl Prim for $ty {
                const VARIANT: Variant = Variant::$ident;
            }
        )*
    };
}

pub trait Prim {
    const VARIANT: Variant;
}

#[repr(u8)]
pub enum Variant {
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    F32,
    F64
}

impl core::fmt::Display for Variant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s: &str = match self {
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::U128 => "u128",
            Self::USize => "usize",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::ISize => "isize",
            Self::F32 => "f32",
            Self::F64 => "f64"
        };
        write!(f, "{}", s)
    }
}

variant!(
    u8 as U8
    u16 as U16
    u32 as U32
    u64 as U64
    u128 as U128
    usize as USize
    i8 as I8
    i16 as I16
    i32 as I32
    i64 as I64
    i128 as I128
    isize as ISize
    f32 as F32
    f64 as F64
);
