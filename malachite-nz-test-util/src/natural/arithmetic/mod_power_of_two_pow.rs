use malachite_base::num::arithmetic::traits::{ModPowerOfTwoMulAssign, ModPowerOfTwoSquareAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::BitIterable;
use malachite_nz::natural::Natural;

pub fn _simple_binary_mod_power_of_two_pow(x: &Natural, exp: &Natural, pow: u64) -> Natural {
    if pow == 0 {
        return Natural::ZERO;
    }
    let mut out = Natural::ONE;
    for bit in exp.bits().rev() {
        out.mod_power_of_two_square_assign(pow);
        if bit {
            out.mod_power_of_two_mul_assign(x, pow);
        }
    }
    out
}