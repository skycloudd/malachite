use integer::Integer;

impl Integer {
    //TODO test
    pub fn count_ones(&self) -> Option<u64> {
        unimplemented!();
    }
}

pub mod from_sign_and_limbs;
pub mod from_twos_complement_limbs;
pub mod get_bit;
pub mod not;
pub mod sign_and_limbs;
pub mod significant_bits;
pub mod trailing_zeros;
pub mod twos_complement_limbs;
