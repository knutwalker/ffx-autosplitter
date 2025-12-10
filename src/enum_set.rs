use core::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct EnumSet<T>(u128, PhantomData<T>);

pub trait EnumSetMember {
    fn ordinal(&self) -> Option<u8>;
}

impl<T: EnumSetMember> EnumSet<T> {
    pub const fn empty() -> Self {
        Self(0, PhantomData)
    }

    pub fn insert(&mut self, item: &T) -> bool {
        let Some(ord) = item.ordinal() else {
            return false;
        };
        if ord >= 128 {
            return false;
        }

        let mask = 1_u128 << ord;
        let previous = self.0 & mask;
        self.0 |= mask;
        return previous == 0;
    }

    pub fn inner(&self) -> u128 {
        return self.0;
    }
}
