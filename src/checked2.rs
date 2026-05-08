use super::*;

pub trait CheckedAdd2
where
    Self: CheckedAdd {
    #[inline]
    fn checked_add(self, rhs: Self) -> Result<Self> {
        CheckedAdd::checked_add(self, rhs).ok_or(Error::Overflow)
    }
}

pub trait CheckedSub2
where
    Self: CheckedSub {
    #[inline]
    fn checked_sub(self, rhs: Self) -> Result<Self> {
        CheckedSub::checked_sub(self, rhs).ok_or(Error::Underflow)
    }
}

pub trait CheckedMul2
where
    Self: CheckedMul {
    fn checked_mul(self, rhs: Self) -> Result<Self> {
        CheckedMul::checked_mul(self, rhs).ok_or(Error::Overflow)
    }
}

pub trait CheckedDiv2
where
    Self: CheckedDiv {
    #[inline]
    fn checked_div(self, rhs: Self) -> Result<Self> {
        CheckedDiv::checked_div(self, rhs).ok_or(Error::DivisionByZero)
    }
}

pub trait CheckedRem2
where
    Self: CheckedRem {
    #[inline]
    fn checked_rem(self, rhs: Self) -> Result<Self> {
        CheckedRem::checked_rem(self, rhs).ok_or(Error::ModuloByZero)
    }
}

pub trait CheckedShl2
where
    Self: CheckedShl {
    #[inline]
    fn checked_shl(self, rhs: u32) -> Result<Self> {
        CheckedShl::checked_shl(self, rhs).ok_or(Error::ShiftOverflow)
    }
}

pub trait CheckedShr2
where
    Self: CheckedShr {
    #[inline]
    fn checked_shr(self, rhs: u32) -> Result<Self> {
        CheckedShr::checked_shr(self, rhs).ok_or(Error::ShiftOverflow)
    }
}

pub trait CheckedNeg2
where
    Self: CheckedNeg {
    #[inline]
    fn checked_neg(self) -> Result<Self> {
        CheckedNeg::checked_neg(self).ok_or(Error::NegationOverflow)
    }
}

impl<T> CheckedAdd2 for T
where
    T: CheckedAdd {}

impl<T> CheckedSub2 for T
where
    T: CheckedSub {}

impl<T> CheckedMul2 for T
where
    T: CheckedMul {}

impl<T> CheckedDiv2 for T
where
    T: CheckedDiv {}

impl<T> CheckedRem2 for T
where
    T: CheckedRem {}

impl<T> CheckedShl2 for T
where
    T: CheckedShl {}

impl<T> CheckedShr2 for T
where
    T: CheckedShr {}

impl<T> CheckedNeg2 for T
where
    T: CheckedNeg {}
