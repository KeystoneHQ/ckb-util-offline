use crate::{H160, H256, H512, H520};

macro_rules! impl_cmp {
    ($name:ident, $bytes_size:expr) => {
        impl ::core::cmp::PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                &self.0[..] == &other.0[..]
            }
        }
        impl ::core::cmp::Eq for $name {}
        impl ::core::cmp::Ord for $name {
            #[inline]
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.0[..].cmp(&other.0[..])
            }
        }
        impl ::core::cmp::PartialOrd for $name {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
    };
}

impl_cmp!(H160, 20);
impl_cmp!(H256, 32);
impl_cmp!(H512, 64);
impl_cmp!(H520, 65);
