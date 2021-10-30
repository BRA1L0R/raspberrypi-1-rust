use crate::bases;

extern "C" {
    pub fn data_barrier();
}

pub trait AddressTraslation {
    fn physical_to_uncachedbus(self) -> Self;
    fn uncachedbus_to_physical(self) -> Self;
}

impl AddressTraslation for u32 {
    fn physical_to_uncachedbus(self) -> Self {
        self + 0xC0000000
    }

    fn uncachedbus_to_physical(self) -> Self {
        self - 0xC0000000
    }
}
