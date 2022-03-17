pub trait UnsignedInt {
    fn bit_width(self) -> u32;
}

macro_rules! impl_unsigned_int {
    ($($t:ty),*) => {
        $(impl UnsignedInt for $t {
            fn bit_width(self) -> u32 {
                Self::BITS - self.leading_zeros()
            }
        })*
    }
}

impl_unsigned_int!(u32, usize);

#[test]
fn test_bit_width() {
    // assert_eq!(0.bit_width(), 0);
}
