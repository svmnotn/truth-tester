use bit_field::BitField;

pub trait State: Clone {
    fn var_at(&self, idx: usize) -> bool;
}

macro_rules! state_num_impl {
    ($($t:ty)*) => ($(
        impl State for $t {
            #[inline]
            fn var_at(&self, idx: usize) -> bool {
                self.get_bit(idx)
            }
        }
    )*)
}

state_num_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

impl State for &[bool] {
    #[inline]
    fn var_at(&self, idx: usize) -> bool {
        self[idx]
    }
}

#[cfg(feature = "parsing")]
use alloc::vec::Vec;
#[cfg(feature = "parsing")]
impl State for Vec<bool> {
    #[inline]
    fn var_at(&self, idx: usize) -> bool {
        self[idx]
    }
}
