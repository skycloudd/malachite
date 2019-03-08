use malachite_base::limbs::limbs_set_zero;
use malachite_base::num::{
    Parity, PrimitiveInteger, UnsignedAbs, WrappingAddAssign, WrappingSubAssign,
};
use natural::arithmetic::add::limbs_add_same_length_to_out;
use natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use natural::arithmetic::add_limb::limbs_add_limb_to_out;
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::mul::limbs_mul_same_length_to_out;
use natural::arithmetic::mul::mul_mod::{
    _limbs_mul_mod_limb_width_to_n_minus_1, _limbs_mul_mod_limb_width_to_n_minus_1_next_size,
    _limbs_mul_mod_limb_width_to_n_minus_1_scratch_size, MULMOD_BNM1_THRESHOLD,
    MUL_FFT_MODF_THRESHOLD,
};
use natural::arithmetic::shl_u::limbs_shl_to_out;
use natural::arithmetic::shl_u::mpn_lshiftc;
use natural::arithmetic::square::SQR_TOOM3_THRESHOLD;
use natural::arithmetic::sub::limbs_sub_same_length_in_place_right;
use natural::arithmetic::sub::{
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_to_out, limbs_sub_to_out,
};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::logic::not::limbs_not_to_out;
use platform::{Limb, SignedLimb};
use std::cmp::{max, Ordering};

pub fn _limbs_mul_greater_to_out_fft_input_sizes_threshold(xs_len: usize, ys_len: usize) -> bool {
    if xs_len == 0 || xs_len < ys_len {
        return false;
    }
    let rn = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(xs_len + ys_len);
    rn.even() && rn >= MULMOD_BNM1_THRESHOLD
}

//TODO tune
pub(crate) const MUL_FFT_THRESHOLD: usize = 4_736;
const SQR_FFT_MODF_THRESHOLD: usize = SQR_TOOM3_THRESHOLD * 3;

//TODO test
// checked
// docs preserved
// Returns smallest possible number of limbs >= pl for a fft of size 2 ^ k, i.e. smallest multiple
// of 2 ^ k >= pl.
// This is mpn_fft_next_size from mpn/generic/mul-fft.c.
pub(crate) fn mpn_fft_next_size(mut pl: usize, k: usize) -> usize {
    pl = 1 + ((pl - 1) >> k); // ceil(pl / 2 ^ k)
    pl << k
}

struct FFTTableNK {
    n: usize,
    k: usize,
}

const MUL_FFT_TABLE3_SIZE: usize = 208;

//TODO tune!!
// from mpn/*/*/gmp-mparam.h
const MUL_FFT_TABLE3: [FFTTableNK; MUL_FFT_TABLE3_SIZE] = [
    FFTTableNK { n: 396, k: 5 },
    FFTTableNK { n: 19, k: 6 },
    FFTTableNK { n: 10, k: 5 },
    FFTTableNK { n: 21, k: 6 },
    FFTTableNK { n: 11, k: 5 },
    FFTTableNK { n: 23, k: 6 },
    FFTTableNK { n: 21, k: 7 },
    FFTTableNK { n: 11, k: 6 },
    FFTTableNK { n: 25, k: 7 },
    FFTTableNK { n: 13, k: 6 },
    FFTTableNK { n: 27, k: 7 },
    FFTTableNK { n: 21, k: 8 },
    FFTTableNK { n: 11, k: 7 },
    FFTTableNK { n: 25, k: 8 },
    FFTTableNK { n: 13, k: 7 },
    FFTTableNK { n: 27, k: 8 },
    FFTTableNK { n: 15, k: 7 },
    FFTTableNK { n: 31, k: 8 },
    FFTTableNK { n: 17, k: 7 },
    FFTTableNK { n: 35, k: 8 },
    FFTTableNK { n: 21, k: 9 },
    FFTTableNK { n: 11, k: 8 },
    FFTTableNK { n: 27, k: 9 },
    FFTTableNK { n: 15, k: 8 },
    FFTTableNK { n: 33, k: 9 },
    FFTTableNK { n: 19, k: 8 },
    FFTTableNK { n: 39, k: 9 },
    FFTTableNK { n: 23, k: 8 },
    FFTTableNK { n: 47, k: 9 },
    FFTTableNK { n: 27, k: 10 },
    FFTTableNK { n: 15, k: 9 },
    FFTTableNK { n: 39, k: 10 },
    FFTTableNK { n: 23, k: 9 },
    FFTTableNK { n: 51, k: 11 },
    FFTTableNK { n: 15, k: 10 },
    FFTTableNK { n: 31, k: 9 },
    FFTTableNK { n: 67, k: 10 },
    FFTTableNK { n: 39, k: 9 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 47, k: 9 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 55, k: 11 },
    FFTTableNK { n: 31, k: 10 },
    FFTTableNK { n: 79, k: 11 },
    FFTTableNK { n: 47, k: 10 },
    FFTTableNK { n: 95, k: 12 },
    FFTTableNK { n: 31, k: 11 },
    FFTTableNK { n: 63, k: 10 },
    FFTTableNK { n: 135, k: 11 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 159, k: 11 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 191, k: 9 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 63, k: 11 },
    FFTTableNK { n: 127, k: 10 },
    FFTTableNK { n: 255, k: 9 },
    FFTTableNK { n: 511, k: 11 },
    FFTTableNK { n: 143, k: 10 },
    FFTTableNK { n: 287, k: 9 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 303, k: 11 },
    FFTTableNK { n: 159, k: 10 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 95, k: 11 },
    FFTTableNK { n: 191, k: 10 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 207, k: 13 },
    FFTTableNK { n: 63, k: 12 },
    FFTTableNK { n: 127, k: 11 },
    FFTTableNK { n: 255, k: 10 },
    FFTTableNK { n: 511, k: 11 },
    FFTTableNK { n: 271, k: 10 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 287, k: 10 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 303, k: 12 },
    FFTTableNK { n: 159, k: 11 },
    FFTTableNK { n: 319, k: 10 },
    FFTTableNK { n: 639, k: 11 },
    FFTTableNK { n: 351, k: 10 },
    FFTTableNK { n: 703, k: 11 },
    FFTTableNK { n: 367, k: 10 },
    FFTTableNK { n: 735, k: 11 },
    FFTTableNK { n: 383, k: 10 },
    FFTTableNK { n: 767, k: 11 },
    FFTTableNK { n: 415, k: 10 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 223, k: 11 },
    FFTTableNK { n: 479, k: 13 },
    FFTTableNK { n: 127, k: 12 },
    FFTTableNK { n: 255, k: 11 },
    FFTTableNK { n: 511, k: 10 },
    FFTTableNK { n: 1023, k: 11 },
    FFTTableNK { n: 543, k: 12 },
    FFTTableNK { n: 287, k: 11 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 1151, k: 11 },
    FFTTableNK { n: 607, k: 12 },
    FFTTableNK { n: 319, k: 11 },
    FFTTableNK { n: 671, k: 12 },
    FFTTableNK { n: 351, k: 11 },
    FFTTableNK { n: 735, k: 12 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 767, k: 12 },
    FFTTableNK { n: 415, k: 11 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 447, k: 11 },
    FFTTableNK { n: 895, k: 12 },
    FFTTableNK { n: 479, k: 14 },
    FFTTableNK { n: 127, k: 13 },
    FFTTableNK { n: 255, k: 12 },
    FFTTableNK { n: 511, k: 11 },
    FFTTableNK { n: 1023, k: 12 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 1151, k: 12 },
    FFTTableNK { n: 607, k: 13 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 735, k: 13 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 831, k: 13 },
    FFTTableNK { n: 447, k: 12 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 255, k: 13 },
    FFTTableNK { n: 511, k: 12 },
    FFTTableNK { n: 1087, k: 13 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 1215, k: 13 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 703, k: 12 },
    FFTTableNK { n: 1407, k: 14 },
    FFTTableNK { n: 383, k: 13 },
    FFTTableNK { n: 767, k: 12 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 511, k: 13 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 2175, k: 13 },
    FFTTableNK { n: 1215, k: 14 },
    FFTTableNK { n: 639, k: 13 },
    FFTTableNK { n: 1343, k: 12 },
    FFTTableNK { n: 2687, k: 13 },
    FFTTableNK { n: 1407, k: 12 },
    FFTTableNK { n: 2815, k: 13 },
    FFTTableNK { n: 1471, k: 14 },
    FFTTableNK { n: 767, k: 13 },
    FFTTableNK { n: 1663, k: 14 },
    FFTTableNK { n: 895, k: 13 },
    FFTTableNK { n: 1919, k: 15 },
    FFTTableNK { n: 511, k: 14 },
    FFTTableNK { n: 1023, k: 13 },
    FFTTableNK { n: 2175, k: 14 },
    FFTTableNK { n: 1151, k: 13 },
    FFTTableNK { n: 2431, k: 12 },
    FFTTableNK { n: 4863, k: 14 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 2687, k: 14 },
    FFTTableNK { n: 1407, k: 13 },
    FFTTableNK { n: 2815, k: 15 },
    FFTTableNK { n: 767, k: 14 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 3071, k: 14 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 3455, k: 14 },
    FFTTableNK { n: 1919, k: 16 },
    FFTTableNK { n: 511, k: 15 },
    FFTTableNK { n: 1023, k: 14 },
    FFTTableNK { n: 2431, k: 13 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 1279, k: 14 },
    FFTTableNK { n: 2943, k: 13 },
    FFTTableNK { n: 5887, k: 15 },
    FFTTableNK { n: 1535, k: 14 },
    FFTTableNK { n: 3455, k: 15 },
    FFTTableNK { n: 1791, k: 14 },
    FFTTableNK { n: 3839, k: 13 },
    FFTTableNK { n: 7679, k: 16 },
    FFTTableNK { n: 1023, k: 15 },
    FFTTableNK { n: 2047, k: 14 },
    FFTTableNK { n: 4223, k: 15 },
    FFTTableNK { n: 2303, k: 14 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 2815, k: 14 },
    FFTTableNK { n: 5887, k: 16 },
    FFTTableNK { n: 1535, k: 15 },
    FFTTableNK { n: 3327, k: 14 },
    FFTTableNK { n: 6911, k: 15 },
    FFTTableNK { n: 3839, k: 14 },
    FFTTableNK { n: 7679, k: 17 },
    FFTTableNK { n: 1023, k: 16 },
    FFTTableNK { n: 2047, k: 15 },
    FFTTableNK { n: 4863, k: 16 },
    FFTTableNK { n: 2559, k: 15 },
    FFTTableNK { n: 5887, k: 14 },
    FFTTableNK { n: 11775, k: 16 },
    FFTTableNK { n: 65536, k: 17 },
    FFTTableNK { n: 131072, k: 18 },
    FFTTableNK { n: 262144, k: 19 },
    FFTTableNK { n: 524288, k: 20 },
    FFTTableNK { n: 1048576, k: 21 },
    FFTTableNK { n: 2097152, k: 22 },
    FFTTableNK { n: 4194304, k: 23 },
    FFTTableNK { n: 8388608, k: 24 },
];

const SQR_FFT_TABLE3_SIZE: usize = 203;

// from mpn/*/*/gmp-mparam.h
const SQR_FFT_TABLE3: [FFTTableNK; SQR_FFT_TABLE3_SIZE] = [
    FFTTableNK { n: 340, k: 5 },
    FFTTableNK { n: 11, k: 4 },
    FFTTableNK { n: 23, k: 5 },
    FFTTableNK { n: 21, k: 6 },
    FFTTableNK { n: 11, k: 5 },
    FFTTableNK { n: 23, k: 6 },
    FFTTableNK { n: 25, k: 7 },
    FFTTableNK { n: 13, k: 6 },
    FFTTableNK { n: 27, k: 7 },
    FFTTableNK { n: 25, k: 8 },
    FFTTableNK { n: 13, k: 7 },
    FFTTableNK { n: 28, k: 8 },
    FFTTableNK { n: 15, k: 7 },
    FFTTableNK { n: 31, k: 8 },
    FFTTableNK { n: 21, k: 9 },
    FFTTableNK { n: 11, k: 8 },
    FFTTableNK { n: 27, k: 9 },
    FFTTableNK { n: 15, k: 8 },
    FFTTableNK { n: 35, k: 9 },
    FFTTableNK { n: 19, k: 8 },
    FFTTableNK { n: 41, k: 9 },
    FFTTableNK { n: 23, k: 8 },
    FFTTableNK { n: 47, k: 9 },
    FFTTableNK { n: 27, k: 10 },
    FFTTableNK { n: 15, k: 9 },
    FFTTableNK { n: 39, k: 10 },
    FFTTableNK { n: 23, k: 9 },
    FFTTableNK { n: 51, k: 11 },
    FFTTableNK { n: 15, k: 10 },
    FFTTableNK { n: 31, k: 9 },
    FFTTableNK { n: 63, k: 10 },
    FFTTableNK { n: 39, k: 9 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 55, k: 11 },
    FFTTableNK { n: 31, k: 10 },
    FFTTableNK { n: 79, k: 11 },
    FFTTableNK { n: 47, k: 10 },
    FFTTableNK { n: 95, k: 12 },
    FFTTableNK { n: 31, k: 11 },
    FFTTableNK { n: 63, k: 10 },
    FFTTableNK { n: 127, k: 9 },
    FFTTableNK { n: 255, k: 10 },
    FFTTableNK { n: 135, k: 11 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 159, k: 9 },
    FFTTableNK { n: 319, k: 11 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 191, k: 9 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 63, k: 11 },
    FFTTableNK { n: 127, k: 10 },
    FFTTableNK { n: 255, k: 9 },
    FFTTableNK { n: 511, k: 10 },
    FFTTableNK { n: 271, k: 9 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 143, k: 10 },
    FFTTableNK { n: 287, k: 9 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 303, k: 9 },
    FFTTableNK { n: 607, k: 11 },
    FFTTableNK { n: 159, k: 10 },
    FFTTableNK { n: 319, k: 9 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 95, k: 11 },
    FFTTableNK { n: 191, k: 10 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 207, k: 13 },
    FFTTableNK { n: 63, k: 12 },
    FFTTableNK { n: 127, k: 11 },
    FFTTableNK { n: 255, k: 10 },
    FFTTableNK { n: 511, k: 11 },
    FFTTableNK { n: 271, k: 10 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 287, k: 10 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 303, k: 10 },
    FFTTableNK { n: 607, k: 11 },
    FFTTableNK { n: 319, k: 10 },
    FFTTableNK { n: 639, k: 11 },
    FFTTableNK { n: 335, k: 10 },
    FFTTableNK { n: 671, k: 11 },
    FFTTableNK { n: 351, k: 10 },
    FFTTableNK { n: 703, k: 11 },
    FFTTableNK { n: 367, k: 12 },
    FFTTableNK { n: 191, k: 11 },
    FFTTableNK { n: 383, k: 10 },
    FFTTableNK { n: 767, k: 11 },
    FFTTableNK { n: 415, k: 10 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 223, k: 11 },
    FFTTableNK { n: 479, k: 13 },
    FFTTableNK { n: 127, k: 12 },
    FFTTableNK { n: 255, k: 11 },
    FFTTableNK { n: 543, k: 12 },
    FFTTableNK { n: 287, k: 11 },
    FFTTableNK { n: 607, k: 12 },
    FFTTableNK { n: 319, k: 11 },
    FFTTableNK { n: 671, k: 12 },
    FFTTableNK { n: 351, k: 11 },
    FFTTableNK { n: 703, k: 13 },
    FFTTableNK { n: 191, k: 12 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 767, k: 12 },
    FFTTableNK { n: 415, k: 11 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 447, k: 11 },
    FFTTableNK { n: 895, k: 12 },
    FFTTableNK { n: 479, k: 14 },
    FFTTableNK { n: 127, k: 13 },
    FFTTableNK { n: 255, k: 12 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 607, k: 13 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 735, k: 13 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 831, k: 13 },
    FFTTableNK { n: 447, k: 12 },
    FFTTableNK { n: 959, k: 13 },
    FFTTableNK { n: 511, k: 12 },
    FFTTableNK { n: 1087, k: 13 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 1215, k: 13 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 1343, k: 13 },
    FFTTableNK { n: 703, k: 12 },
    FFTTableNK { n: 1407, k: 14 },
    FFTTableNK { n: 383, k: 13 },
    FFTTableNK { n: 767, k: 12 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 511, k: 13 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 2175, k: 13 },
    FFTTableNK { n: 1215, k: 14 },
    FFTTableNK { n: 639, k: 13 },
    FFTTableNK { n: 1343, k: 12 },
    FFTTableNK { n: 2687, k: 13 },
    FFTTableNK { n: 1407, k: 12 },
    FFTTableNK { n: 2815, k: 13 },
    FFTTableNK { n: 1471, k: 14 },
    FFTTableNK { n: 767, k: 13 },
    FFTTableNK { n: 1599, k: 12 },
    FFTTableNK { n: 3199, k: 13 },
    FFTTableNK { n: 1663, k: 14 },
    FFTTableNK { n: 895, k: 13 },
    FFTTableNK { n: 1791, k: 15 },
    FFTTableNK { n: 511, k: 14 },
    FFTTableNK { n: 1023, k: 13 },
    FFTTableNK { n: 2175, k: 14 },
    FFTTableNK { n: 1151, k: 13 },
    FFTTableNK { n: 2431, k: 12 },
    FFTTableNK { n: 4863, k: 14 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 2687, k: 14 },
    FFTTableNK { n: 1407, k: 13 },
    FFTTableNK { n: 2815, k: 15 },
    FFTTableNK { n: 767, k: 14 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 3199, k: 14 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 3455, k: 12 },
    FFTTableNK { n: 6911, k: 14 },
    FFTTableNK { n: 1791, k: 16 },
    FFTTableNK { n: 511, k: 15 },
    FFTTableNK { n: 1023, k: 14 },
    FFTTableNK { n: 2431, k: 13 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 1279, k: 14 },
    FFTTableNK { n: 2943, k: 13 },
    FFTTableNK { n: 5887, k: 15 },
    FFTTableNK { n: 1535, k: 14 },
    FFTTableNK { n: 3455, k: 13 },
    FFTTableNK { n: 6911, k: 15 },
    FFTTableNK { n: 1791, k: 14 },
    FFTTableNK { n: 3839, k: 16 },
    FFTTableNK { n: 1023, k: 15 },
    FFTTableNK { n: 2047, k: 14 },
    FFTTableNK { n: 4223, k: 15 },
    FFTTableNK { n: 2303, k: 14 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 2815, k: 14 },
    FFTTableNK { n: 5887, k: 16 },
    FFTTableNK { n: 1535, k: 15 },
    FFTTableNK { n: 3327, k: 14 },
    FFTTableNK { n: 6911, k: 15 },
    FFTTableNK { n: 3839, k: 17 },
    FFTTableNK { n: 1023, k: 16 },
    FFTTableNK { n: 2047, k: 15 },
    FFTTableNK { n: 4863, k: 16 },
    FFTTableNK { n: 2559, k: 15 },
    FFTTableNK { n: 5887, k: 14 },
    FFTTableNK { n: 11775, k: 16 },
    FFTTableNK { n: 65536, k: 17 },
    FFTTableNK { n: 131072, k: 18 },
    FFTTableNK { n: 262144, k: 19 },
    FFTTableNK { n: 524288, k: 20 },
    FFTTableNK { n: 1048576, k: 21 },
    FFTTableNK { n: 2097152, k: 22 },
    FFTTableNK { n: 4194304, k: 23 },
    FFTTableNK { n: 8388608, k: 24 },
];

//TODO test
// checked
// docs preserved
// Find the best k to use for a mod 2 ^ (m * Limb::WIDTH) + 1 FFT for m >= n. We have sqr = 0 if for
// a multiply, sqr = 1 for a square.
// mpn_fft_best_k from mpn/generic/mul_fft.c, mpn_fft_table3 variant
pub(crate) fn mpn_fft_best_k(n: usize, sqr: bool) -> usize {
    let fft_tab: &[FFTTableNK] = if sqr {
        &SQR_FFT_TABLE3
    } else {
        &MUL_FFT_TABLE3
    };
    let mut last_k = fft_tab[0].k;
    let mut tab = 1;
    loop {
        let tab_n = fft_tab[tab].n;
        let thres = tab_n << last_k;
        if n <= thres as usize {
            break;
        }
        last_k = fft_tab[tab].k;
        tab += 1;
    }
    last_k
}

// This is mpn_fft_mul from gmp-impl.h.
#[inline]
pub fn _limbs_mul_greater_to_out_fft(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    mpn_nussbaumer_mul(out, xs, ys);
}

// This is mpn_nussbaumer_mul from mpn/generic/mpn_nussbaumer_mul.c.
fn mpn_nussbaumer_mul(pp: &mut [Limb], ap: &[Limb], bp: &[Limb]) {
    let an = ap.len();
    let bn = bp.len();
    assert!(an >= bn);
    assert_ne!(bn, 0);

    //TODO special case for squaring
    let rn = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(an + bn);
    let mut tp = vec![0; _limbs_mul_mod_limb_width_to_n_minus_1_scratch_size(rn, an, bn)];
    _limbs_mul_mod_limb_width_to_n_minus_1(pp, rn, ap, bp, &mut tp);
}

// Initialize l[i][j] with bitrev(j)
// This is mpn_fft_initl from mpn/generic/mul_fft.c.
fn mpn_fft_initl(l: &mut [&mut [usize]], k: usize) {
    l[0][0] = 0;
    let mut i = 1;
    let mut big_k = 1;
    while i <= k {
        for j in 0..big_k {
            l[i][j] = 2 * l[i - 1][j];
            l[i][big_k + j] = 1 + l[i][j];
        }
        i += 1;
        big_k <<= 1;
    }
}

// return the lcm of a and 2^k
// This is mpn_mul_fft_lcm from mpn/generic/mul_fft.c.
fn mpn_mul_fft_lcm(mut a: usize, mut k: usize) -> usize {
    let l = k;
    while a % 2 == 0 && k > 0 {
        a >>= 1;
        k -= 1;
    }
    a << l
}

// r <- a*2^d mod 2^(n*`Limb::WIDTH`)+1 with a = {a, n+1}
// Assumes a is semi-normalized, i.e. a[n] <= 1.
// r and a must have n+1 limbs, and not overlap.
// This is mpn_fft_mul_2exp_modF from mpn/generic/mul_fft.c.
fn mpn_fft_mul_2exp_mod_f(r: &mut [Limb], a: &[Limb], d: usize, n: usize) {
    let sh = d as u32 % Limb::WIDTH;
    let mut m = d / Limb::WIDTH as usize;
    // negate
    if m >= n {
        // r[0..m-1]  <-- lshift(a[n-m]..a[n-1], sh)
        // r[m..n-1]  <-- -lshift(a[0]..a[n-m-1],  sh)

        m -= n;
        let mut cc;
        let mut rd;
        if sh != 0 {
            // no out shift below since a[n] <= 1
            limbs_shl_to_out(r, &a[n - m..n + 1], sh);
            rd = r[m];
            cc = mpn_lshiftc(&mut r[m..], &a[..n - m], sh);
        } else {
            r[..m].copy_from_slice(&a[n - m..n]);
            rd = a[n];
            limbs_not_to_out(&mut r[m..], &a[..n - m]);
            cc = 0;
        }

        // add cc to r[0], and add rd to r[m]
        // now add 1 in r[m], subtract 1 in r[n], i.e. add 1 in r[0]
        r[n] = 0;
        // cc < 2^sh <= 2^(Limb::WIDTH`-1) thus no overflow here
        cc += 1;
        limbs_slice_add_limb_in_place(r, cc);
        rd.wrapping_add_assign(1);
        // rd might overflow when sh=Limb::WIDTH`-1
        cc = if rd == 0 { 1 } else { rd };
        limbs_slice_add_limb_in_place(&mut r[m + if rd == 0 { 1 } else { 0 }..], cc);
    } else {
        let mut cc;
        let rd;
        // r[0..m-1]  <-- -lshift(a[n-m]..a[n-1], sh)
        // r[m..n-1]  <-- lshift(a[0]..a[n-m-1],  sh)
        if sh != 0 {
            // no out bits below since a[n] <= 1
            mpn_lshiftc(r, &a[n - m..n + 1], sh);
            rd = !r[m];
            // {r, m+1} = {a+n-m, m+1} << sh
            cc = limbs_shl_to_out(&mut r[m..], &a[..n - m], sh); // {r+m, n-m} = {a, n-m}<<sh
        } else {
            // r[m] is not used below, but we save a test for m=0
            limbs_not_to_out(r, &a[n - m..n + 1]);
            rd = a[n];
            r[m..n].copy_from_slice(&a[..n - m]);
            cc = 0;
        }

        // now complement {r, m}, subtract cc from r[0], subtract rd from r[m]
        // if m=0 we just have r[0]=a[n] << sh
        if m != 0 {
            // now add 1 in r[0], subtract 1 in r[m]
            // then add 1 to r[0]
            if cc == 0 {
                cc = if limbs_slice_add_limb_in_place(&mut r[..n], 1) {
                    1
                } else {
                    0
                };
            } else {
                cc -= 1;
            }
            cc = if limbs_sub_limb_in_place(&mut r[..m], cc) {
                1
            } else {
                0
            } + 1;
            // add 1 to cc instead of rd since rd might overflow
        }

        // now subtract cc and rd from r[m..n]
        let (r_last, r_init) = r[..n + 1].split_last_mut().unwrap();
        *r_last = (if limbs_sub_limb_in_place(&mut r_init[m..], cc) {
            1 as Limb
        } else {
            0
        })
        .wrapping_neg();
        r_last.wrapping_sub_assign(if limbs_sub_limb_in_place(&mut r_init[m..], rd) {
            1
        } else {
            0
        });
        if r_last.get_highest_bit() {
            *r_last = if limbs_slice_add_limb_in_place(r_init, 1) {
                1
            } else {
                0
            };
        }
    }
}

// store in A[0..nprime] the first M bits from {n, nl},
// in A[nprime+1..] the following M bits, ...
// Assumes M is a multiple of GMP_NUMB_BITS (M = l * GMP_NUMB_BITS).
// T must have space for at least (nprime + 1) limbs.
// We must have nl <= 2*k*l.
// This is mpn_mul_fft_decompose from mpn/generic/mul_fft.c.
fn mpn_mul_fft_decompose<'a>(
    a: &'a mut [Limb],
    k: usize,
    nprime: usize,
    n: &[Limb],
    mut nl: usize,
    l: usize,
    mp: usize,
    t: &mut [Limb],
) -> Vec<&'a mut [Limb]> {
    let kl = k * l;

    // normalize {n, nl} mod 2^(Kl*GMP_NUMB_BITS)+1
    let mut n_is_tmp = false;
    let mut n_offset = 0;
    let mut cy: SignedLimb;
    let mut tmp;
    if nl > kl {
        let dif = nl - kl;
        tmp = vec![0; kl + 1];
        // dif > kl -> nl - k * l > k * l -> nl > 2 * k * l, which violates the precondition
        // nl <= 2 * k * l. So dif > kl cannot happen.
        assert!(dif <= kl);
        // dif <= Kl, i.e. nl <= 2 * Kl
        cy = if limbs_sub_to_out(&mut tmp, &n[..kl], &n[kl..kl + dif]) {
            1
        } else {
            0
        };
        cy = if limbs_slice_add_limb_in_place(&mut tmp[..kl], cy as Limb) {
            1
        } else {
            0
        };
        tmp[kl] = cy as Limb;
        nl = kl + 1;
        n_is_tmp = true;
    } else {
        tmp = Vec::with_capacity(0);
    }
    let mut ap: Vec<&mut [Limb]> = Vec::with_capacity(k);
    let mut tmp_offset = 0;
    let mut remainder: &mut [Limb] = a;
    for i in 0..k {
        // force remainder to move rather than be borrowed
        let (a_lo, a_hi) = { remainder }.split_at_mut(nprime + 1);
        remainder = a_hi;
        // store the next M bits of n into A[0..nprime]
        // nl is the number of remaining limbs
        if nl > 0 {
            let j = if l <= nl && i < k - 1 { l } else { nl }; // store j next limbs
            nl -= j;
            if n_is_tmp {
                t[..j].copy_from_slice(&tmp[tmp_offset..tmp_offset + j]);
            } else {
                t[..j].copy_from_slice(&n[n_offset..n_offset + j]);
            }
            limbs_set_zero(&mut t[j..nprime + 1]);
            if n_is_tmp {
                tmp_offset += l;
            } else {
                n_offset += l;
            }
            mpn_fft_mul_2exp_mod_f(a_lo, t, i * mp, nprime);
        } else {
            limbs_set_zero(a_lo);
        }
        ap.push(a_lo);
    }
    assert_eq!(nl, 0);
    ap
}

// This is mpn_fft_add_modF from mpn/generic/mul_fft.c, where r == a.
fn mpn_fft_add_mod_f_in_place_left(a: &mut [Limb], b: &[Limb], n: usize) {
    let c = a[n].wrapping_add(b[n]).wrapping_add(
        if limbs_slice_add_same_length_in_place_left(&mut a[..n], &b[..n]) {
            1
        } else {
            0
        },
    );
    // 0 <= c <= 3
    if c > 1 {
        a[n] = 1; // r[n] - c = 1
        assert!(!limbs_sub_limb_in_place(&mut a[..n + 1], c - 1));
    } else {
        a[n] = c;
    }
}

// r <- a-b mod 2^(n*GMP_NUMB_BITS)+1.
// Assumes a and b are semi-normalized.
// This is mpn_fft_sub_modF from mpn/generic/mul_fft.c.
fn mpn_fft_sub_mod_f(r: &mut [Limb], a: &[Limb], b: &[Limb], n: usize) {
    let c = a[n].wrapping_sub(b[n]).wrapping_sub(
        if limbs_sub_same_length_to_out(r, &a[..n], &b[..n]) {
            1
        } else {
            0
        },
    );
    // -2 <= c <= 1
    if c.get_highest_bit() {
        r[n] = 0;
        assert!(!limbs_slice_add_limb_in_place(
            &mut r[..n + 1],
            c.wrapping_neg()
        ));
    } else {
        r[n] = c;
    }
}

// input: A[0] ... A[inc*(K-1)] are residues mod 2^N+1 where
//    N=n*GMP_NUMB_BITS, and 2^omega is a primitive root mod 2^N+1
// output: A[inc*l[k][i]] <- \sum (2^omega)^(ij) A[inc*j] mod 2^N+1
// This is mpn_fft_fft from mpn/generic/mul_fft.c.
pub fn mpn_fft_fft(
    ap: &mut [&mut [Limb]],
    k: usize,
    ll: &[&[usize]],
    ll_offset: usize,
    omega: usize,
    n: usize,
    inc: usize,
    tp: &mut [Limb],
) {
    if k == 2 {
        tp[..n + 1].copy_from_slice(&ap[0][..n + 1]);
        {
            let (ap_first, ap_tail) = ap.split_first_mut().unwrap();
            limbs_slice_add_same_length_in_place_left(
                &mut ap_first[..n + 1],
                &ap_tail[inc - 1][..n + 1],
            );
        }
        let cy = limbs_sub_same_length_in_place_right(&tp[..n + 1], &mut ap[inc][..n + 1]);
        // can be 2 or 3
        if ap[0][n] > 1 {
            let x = ap[0][n] - 1;
            ap[0][n] = 1 - if limbs_sub_limb_in_place(&mut ap[0][..n], x) {
                1
            } else {
                0
            };
        }
        // Ap[inc][n] can be -1 or -2
        if cy {
            let x = (!ap[inc][n]).wrapping_add(1);
            ap[inc][n] = if limbs_slice_add_limb_in_place(&mut ap[inc][..n], x) {
                1
            } else {
                0
            };
        }
    } else {
        let k2 = k >> 1;
        let mut lki = 0;

        mpn_fft_fft(ap, k2, ll, ll_offset - 1, 2 * omega, n, inc * 2, tp);
        mpn_fft_fft(
            &mut ap[inc..],
            k2,
            ll,
            ll_offset - 1,
            2 * omega,
            n,
            inc * 2,
            tp,
        );
        let mut ap_offset = 0;
        //  A[2*j*inc]   <- A[2*j*inc] + omega^l[k][2*j*inc] A[(2j+1)inc]
        // A[(2j+1)inc] <- A[2*j*inc] + omega^l[k][(2j+1)inc] A[(2j+1)inc]
        for _ in 0..k2 {
            /* Ap[inc] <- Ap[0] + Ap[inc] * 2^(lk[1] * omega)
            Ap[0]   <- Ap[0] + Ap[inc] * 2^(lk[0] * omega) */
            mpn_fft_mul_2exp_mod_f(
                tp,
                ap[ap_offset + inc],
                ll[ll_offset][lki].wrapping_mul(omega),
                n,
            );
            {
                let (ap_lo, ap_hi) = ap.split_at_mut(ap_offset + inc);
                mpn_fft_sub_mod_f(ap_hi[0], ap_lo[ap_offset], tp, n);
            }
            mpn_fft_add_mod_f_in_place_left(ap[ap_offset], tp, n);
            lki += 2;
            ap_offset += 2 * inc;
        }
    }
}

// input: A^[l[k][0]] A^[l[k][1]] ... A^[l[k][K-1]]
// output: K*A[0] K*A[K-1] ... K*A[1].
// Assumes the Ap[] are pseudo-normalized, i.e. 0 <= Ap[][n] <= 1.
// This condition is also fulfilled at exit.
// This is mpn_fft_fftinv from mpn/generic/mul_fft.c.
pub fn mpn_fft_fftinv(ap: &mut [&mut [Limb]], k: usize, omega: usize, n: usize, tp: &mut [Limb]) {
    if k == 2 {
        tp[..n + 1].copy_from_slice(&ap[0][..n + 1]);
        {
            let (ap_first, ap_tail) = ap.split_first_mut().unwrap();
            limbs_slice_add_same_length_in_place_left(&mut ap_first[..n + 1], &ap_tail[0][..n + 1]);
        }
        let cy = limbs_sub_same_length_in_place_right(&tp[..n + 1], &mut ap[1][..n + 1]);
        // can be 2 or 3
        if ap[0][n] > 1 {
            let x = ap[0][n] - 1;
            ap[0][n] = 1 - if limbs_sub_limb_in_place(&mut ap[0][..n], x) {
                1
            } else {
                0
            };
        }
        // Ap[1][n] can be -1 or -2
        if cy {
            let x = (!ap[1][n]).wrapping_add(1);
            ap[1][n] = if limbs_slice_add_limb_in_place(&mut ap[1][..n], x) {
                1
            } else {
                0
            };
        }
    } else {
        let k2 = k >> 1;
        mpn_fft_fftinv(ap, k2, 2 * omega, n, tp);
        mpn_fft_fftinv(&mut ap[k2..], k2, 2 * omega, n, tp);
        // A[j]     <- A[j] + omega^j A[j+K/2]
        // A[j+K/2] <- A[j] + omega^(j+K/2) A[j+K/2]
        let mut ap_offset = 0;
        for j in 0..k2 {
            // Ap[K2] <- Ap[0] + Ap[K2] * 2^((j + K2) * omega)
            // Ap[0]  <- Ap[0] + Ap[K2] * 2^(j * omega)
            mpn_fft_mul_2exp_mod_f(tp, ap[ap_offset + k2], j * omega, n);
            {
                let (ap_lo, ap_hi) = ap.split_at_mut(ap_offset + k2);
                mpn_fft_sub_mod_f(ap_hi[0], ap_lo[ap_offset], tp, n);
            }
            mpn_fft_add_mod_f_in_place_left(ap[ap_offset], tp, n);
            ap_offset += 1;
        }
    }
}

// Given ap[0..n] with ap[n]<=1, reduce it modulo 2^(n*GMP_NUMB_BITS)+1,
// by subtracting that modulus if necessary.
//
// If ap[0..n] is exactly 2^(n*GMP_NUMB_BITS) then mpn_sub_1 produces a
// borrow and the limbs must be zeroed out again.  This will occur very
// infrequently.
// This is mpn_fft_normalize from mpn/generic/mul_fft.c.
fn mpn_fft_normalize(ap: &mut [Limb], n: usize) {
    if ap[n] != 0 {
        assert!(!limbs_sub_limb_in_place(&mut ap[..n + 1], 1));
        if ap[n] == 0 {
            // This happens with very low probability; we have yet to trigger it,
            // and thereby make sure this code is correct.
            limbs_set_zero(&mut ap[..n]);
            ap[n] = 1;
        } else {
            ap[n] = 0;
        }
    }
}

// R <- A/2^k mod 2^(n*GMP_NUMB_BITS)+1
// This is mpn_fft_div_2exp_modF from mpn/generic/mul_fft.c.
fn mpn_fft_div_2exp_mod_f(r: &mut [Limb], a: &[Limb], k: usize, n: usize) {
    assert!(r.len() >= n + 1);
    let i = 2 * n * Limb::WIDTH as usize - k;
    mpn_fft_mul_2exp_mod_f(r, a, i, n);
    // 1/2^k = 2^(2nL-k) mod 2^(n*GMP_NUMB_BITS)+1
    // normalize so that R < 2^(n*GMP_NUMB_BITS)+1
    mpn_fft_normalize(r, n);
}

// {rp,n} <- {ap,an} mod 2^(n*GMP_NUMB_BITS)+1, n <= an <= 3*n.
// Returns carry out, i.e. 1 iff {ap,an} = -1 mod 2^(n*GMP_NUMB_BITS)+1,
// then {rp,n}=0.
// This is mpn_fft_norm_modF from mpn/generic/mul_fft.c.
pub fn mpn_fft_norm_mod_f(rp: &mut [Limb], n: usize, ap: &[Limb], an: usize) -> Limb {
    assert!(n <= an && an <= 3 * n);
    let m = an as isize - 2 * n as isize;
    let l;
    let mut rpn: SignedLimb;
    if m > 0 {
        let m = m as usize;
        l = n;
        // add {ap, m} and {ap+2n, m} in {rp, m}
        let cc = if limbs_add_same_length_to_out(rp, &ap[..m], &ap[2 * n..2 * n + m]) {
            1
        } else {
            0
        };
        // copy {ap+m, n-m} to {rp+m, n-m}
        rpn = if limbs_add_limb_to_out(&mut rp[m..n], &ap[m..n], cc) {
            1
        } else {
            0
        };
    } else {
        l = an - n; // l <= n
        rp[..n].copy_from_slice(&ap[..n]);
        rpn = 0;
    }
    // remains to subtract {ap+n, l} from {rp, n+1}
    let cc = if limbs_sub_same_length_in_place_left(&mut rp[..l], &ap[n..n + l]) {
        1
    } else {
        0
    };
    rpn -= if limbs_sub_limb_in_place(&mut rp[l..n], cc) {
        1
    } else {
        0
    };
    // necessarily rpn = -1
    if rpn < 0 {
        if limbs_slice_add_limb_in_place(&mut rp[..n], 1) {
            1
        } else {
            0
        }
    } else {
        rpn.unsigned_abs()
    }
}

// This is mpn_fft_mul_modF_K from mpn/generic/mul_fft.c, where ap != bp.
fn mpn_fft_mul_mod_f_k(ap: &mut [&mut [Limb]], bp: &mut [&mut [Limb]], n: usize, big_k: usize) {
    if n >= MUL_FFT_MODF_THRESHOLD {
        let k = mpn_fft_best_k(n, false);
        let k2 = 1 << k;
        assert_eq!(n & (k2 - 1), 0);
        let max_lk = max(k2, Limb::WIDTH as usize);
        let m2 = n * Limb::WIDTH as usize >> k;
        let l = n >> k;
        let mut big_nprime2 = (2 * m2 + k + 2 + max_lk) / max_lk * max_lk;
        // Nprime2 = ceil((2*M2+k+3)/maxLK)*maxLK
        let mut nprime2 = big_nprime2 / Limb::WIDTH as usize;

        // we should ensure that nprime2 is a multiple of the next K
        if nprime2 >= MUL_FFT_MODF_THRESHOLD {
            loop {
                let k3 = 1 << mpn_fft_best_k(nprime2, false);
                if nprime2 & (k3 - 1) == 0 {
                    break;
                }
                nprime2 = (nprime2 + k3 - 1) & k3.wrapping_neg();
                big_nprime2 = nprime2 * Limb::WIDTH as usize;
                // warning: since nprime2 changed, K3 may change too!
            }
        }
        assert!(nprime2 < n); // otherwise we'll loop

        let mp2 = big_nprime2 >> k;

        let mut a = vec![0; 2 * (nprime2 + 1) << k];
        let (a, b) = a.split_at_mut((nprime2 + 1) << k);
        let mut t = vec![0; 2 * (nprime2 + 1)];
        let mut tmp = vec![0; 2 << k];
        let mut remainder: &mut [usize] = &mut tmp;
        let mut fft_l = Vec::with_capacity(k + 1);
        for i in 0..k + 1 {
            // force remainder to move rather than be borrowed
            let (tmp_lo, tmp_hi) = { remainder }.split_at_mut(1 << i);
            fft_l.push(tmp_lo);
            remainder = tmp_hi;
        }
        mpn_fft_initl(&mut fft_l, k);
        let mut api = 0;
        let mut bpi = 0;
        let immut_fft_l: Vec<&[usize]> = fft_l.into_iter().map(|row| &*row).collect();
        for _ in 0..big_k {
            mpn_fft_normalize(ap[api], n);
            mpn_fft_normalize(bp[bpi], n);
            let mut big_ap =
                mpn_mul_fft_decompose(a, k2, nprime2, ap[api], (l << k) + 1, l, mp2, &mut t);
            mpn_mul_fft_decompose(b, k2, nprime2, bp[bpi], (l << k) + 1, l, mp2, &mut t);
            let cy = mpn_mul_fft_internal(
                ap[api],
                n,
                k,
                big_ap,
                b,
                nprime2,
                l,
                mp2,
                &immut_fft_l,
                &mut t,
                false,
            );
            ap[api][n] = cy;
            api += 1;
            bpi += 1;
        }
    } else {
        let n2 = 2 * n;
        let mut tp = vec![0; n2];
        let mut api = 0;
        let mut bpi = 0;
        for _ in 0..big_k {
            let a = &mut ap[api];
            let b = &mut bp[bpi];
            api += 1;
            bpi += 1;
            limbs_mul_same_length_to_out(&mut tp, &b[..n], &a[..n]);
            let mut cc = if a[n] != 0 {
                if limbs_slice_add_same_length_in_place_left(&mut tp[n..2 * n], &b[..n]) {
                    1
                } else {
                    0
                }
            } else {
                0
            };
            if b[n] != 0 {
                cc += if limbs_slice_add_same_length_in_place_left(&mut tp[n..2 * n], &a[..n]) {
                    1
                } else {
                    0
                } + a[n];
            }
            if cc != 0 {
                assert!(!limbs_slice_add_limb_in_place(&mut tp[..n2], cc));
            }
            a[n] = if limbs_sub_same_length_to_out(a, &tp[..n], &tp[n..2 * n])
                && limbs_slice_add_limb_in_place(&mut a[..n], 1)
            {
                1
            } else {
                0
            };
        }
    }
}

// This is mpn_fft_mul_modF_K from mpn/generic/mul_fft.c.
fn mpn_fft_mul_mod_f_k_sqr(ap: &mut [&mut [Limb]], n: usize, big_k: usize) {
    if n >= SQR_FFT_MODF_THRESHOLD {
        let k = mpn_fft_best_k(n, false);
        let k2 = 1 << k;
        assert_eq!(n & (k2 - 1), 0);
        let max_lk = max(k2, Limb::WIDTH as usize);
        let m2 = n * Limb::WIDTH as usize >> k;
        let l = n >> k;
        let mut big_nprime2 = (2 * m2 + k + 2 + max_lk) / max_lk * max_lk;
        // Nprime2 = ceil((2*M2+k+3)/maxLK)*maxLK
        let mut nprime2 = big_nprime2 / Limb::WIDTH as usize;

        // we should ensure that nprime2 is a multiple of the next K
        if nprime2 >= SQR_FFT_MODF_THRESHOLD {
            //mp_size_t K3;
            loop {
                let k3 = 1 << mpn_fft_best_k(nprime2, true);
                if nprime2 & (k3 - 1) == 0 {
                    break;
                }
                nprime2 = (nprime2 + k3 - 1) & k3.wrapping_neg();
                big_nprime2 = nprime2 * Limb::WIDTH as usize;
                // warning: since nprime2 changed, K3 may change too!
            }
        }
        assert!(nprime2 < n); // otherwise we'll loop

        let mp2 = big_nprime2 >> k;
        let mut a = vec![0; 2 * (nprime2 + 1) << k];
        let (a_lo, a_hi) = a.split_at_mut((nprime2 + 1) << k);
        let mut t = vec![0; 2 * (nprime2 + 1)];
        let mut tmp = vec![0; 2 << k];
        let mut remainder: &mut [usize] = &mut tmp;
        let mut fft_l = Vec::with_capacity(k + 1);
        for i in 0..k + 1 {
            // force remainder to move rather than be borrowed
            let (tmp_lo, tmp_hi) = { remainder }.split_at_mut(1 << i);
            fft_l.push(tmp_lo);
            remainder = tmp_hi;
        }
        mpn_fft_initl(&mut fft_l, k);
        let mut api = 0;
        let immut_fft_l: Vec<&[usize]> = fft_l.into_iter().map(|row| &*row).collect();
        for _ in 0..big_k {
            mpn_fft_normalize(ap[api], n);
            let mut big_ap =
                mpn_mul_fft_decompose(a_lo, k2, nprime2, ap[api], (l << k) + 1, l, mp2, &mut t);
            let cy = mpn_mul_fft_internal(
                ap[api],
                n,
                k,
                big_ap,
                a_hi,
                nprime2,
                l,
                mp2,
                &immut_fft_l,
                &mut t,
                true,
            );
            ap[api][n] = cy;
            api += 1;
        }
    } else {
        let n2 = 2 * n;
        let mut tp = vec![0; n2];
        let mut api = 0;
        for _ in 0..big_k {
            let a = &mut ap[api];
            api += 1;
            //TODO use square
            limbs_mul_same_length_to_out(&mut tp, &a[..n], &a[..n]);
            let mut cc = if a[n] != 0 {
                if limbs_slice_add_same_length_in_place_left(&mut tp[n..2 * n], &a[..n]) {
                    1
                } else {
                    0
                }
            } else {
                0
            };
            if a[n] != 0 {
                cc += if limbs_slice_add_same_length_in_place_left(&mut tp[n..2 * n], &a[..n]) {
                    1
                } else {
                    0
                } + a[n];
            }
            if cc != 0 {
                assert!(!limbs_slice_add_limb_in_place(&mut tp[..n2], cc));
            }
            a[n] = if limbs_sub_same_length_to_out(a, &tp[..n], &tp[n..2 * n])
                && limbs_slice_add_limb_in_place(&mut a[..n], 1)
            {
                1
            } else {
                0
            };
        }
    }
}

// This is mpn_mul_fft_internal from mpn/generic/mul_fft.c. A is excluded as it is unused.
pub fn mpn_mul_fft_internal(
    op: &mut [Limb],
    pl: usize,
    k: usize,
    mut ap: Vec<&mut [Limb]>,
    b: &mut [Limb],
    nprime: usize,
    l: usize,
    mp: usize,
    fft_l: &[&[usize]],
    t: &mut [Limb],
    sqr: bool,
) -> Limb {
    let big_k = 1usize << k;
    {
        let mut bp: Vec<&mut [Limb]> = Vec::with_capacity(big_k);
        let mut remainder: &mut [Limb] = b;
        for _ in 0..big_k {
            // force remainder to move rather than be borrowed
            let (b_lo, b_hi) = { remainder }.split_at_mut(nprime + 1);
            bp.push(b_lo);
            remainder = b_hi;
        }
        // direct fft's
        mpn_fft_fft(&mut ap, big_k, fft_l, k, 2 * mp, nprime, 1, t);
        if !sqr {
            mpn_fft_fft(&mut bp, big_k, fft_l, k, 2 * mp, nprime, 1, t);
        }

        // term to term multiplications
        if sqr {
            mpn_fft_mul_mod_f_k_sqr(&mut ap, nprime, big_k);
        } else {
            mpn_fft_mul_mod_f_k(&mut ap, &mut bp, nprime, big_k);
        }
    }

    // inverse fft's
    mpn_fft_fftinv(&mut ap, big_k, 2 * mp, nprime, t);

    // division of terms after inverse fft
    let mut bp: Vec<&mut [Limb]> = Vec::with_capacity(big_k);
    let (t_lo, t_hi) = t.split_at_mut(nprime + 1);
    bp.push(t_hi);
    mpn_fft_div_2exp_mod_f(&mut bp[0], &mut ap[0], k, nprime);

    for i in 1..big_k {
        let (ap_lo, ap_hi) = ap.split_at_mut(i);
        mpn_fft_div_2exp_mod_f(
            &mut ap_lo[i - 1],
            &mut ap_hi[0],
            k + (big_k - i) * mp,
            nprime,
        );
    }
    bp.extend(ap.drain(..big_k - 1));

    // addition of terms in result p
    limbs_set_zero(t_lo);
    let pla = l * (big_k - 1) + nprime + 1; // number of required limbs for p

    // B has K*(n' + 1) limbs, which is >= pla, i.e. enough
    limbs_set_zero(&mut b[..pla]);
    let mut cc: SignedLimb = 0; // will accumulate the (signed) carry at p[pla]
    let mut i = big_k - 1;
    let mut lo = l * i + nprime;
    let mut sh = l * i;
    loop {
        let j;
        {
            let n = &mut b[sh..];
            j = (big_k - i) & (big_k - 1);
            if limbs_slice_add_same_length_in_place_left(&mut n[..nprime + 1], &bp[j][..nprime + 1])
            {
                cc += if limbs_slice_add_limb_in_place(&mut n[nprime + 1..pla - sh], 1) {
                    1
                } else {
                    0
                };
            }
            if 2 * l < t_lo.len() {
                t_lo[2 * l] = i as Limb + 1; // T = (i + 1)*2^(2*M)
            } else {
                bp[0][2 * l - t_lo.len()] = i as Limb + 1; // T = (i + 1)*2^(2*M)
            }
        }
        if limbs_cmp_same_length(&bp[j][..nprime + 1], t_lo) == Ordering::Greater {
            // subtract 2^N'+1
            {
                let n = &mut b[sh..];
                cc -= if limbs_sub_limb_in_place(&mut n[..pla - sh], 1) {
                    1
                } else {
                    0
                };
            }
            cc -= if limbs_sub_limb_in_place(&mut b[lo..pla], 1) {
                1
            } else {
                0
            };
        }

        if i == 0 {
            break;
        }
        i -= 1;
        lo -= l;
        sh -= l;
    }
    if cc == -1 {
        cc = if limbs_slice_add_limb_in_place(&mut b[pla - pl..pla], 1) {
            1
        } else {
            0
        };
        if cc != 0 {
            // p[pla-pl]...p[pla-1] are all zero
            limbs_sub_limb_in_place(&mut b[pla - pl - 1..pla], 1);
            limbs_sub_limb_in_place(&mut b[pla - 1..pla], 1);
        }
    } else if cc == 1 {
        // This branch is untested!
        let mut cc = 1 as Limb;
        if pla >= 2 * pl {
            loop {
                cc = if limbs_slice_add_limb_in_place(&mut b[pla - 2 * pl..pla], cc) {
                    1
                } else {
                    0
                };
                if cc == 0 {
                    break;
                }
            }
        } else {
            cc = if limbs_sub_limb_in_place(&mut b[pla - pl..pla], cc) {
                1
            } else {
                0
            };
            assert_eq!(cc, 0);
        }
    } else {
        assert_eq!(cc, 0);
    }
    // here p < 2^(2M) [K 2^(M(K-1)) + (K-1) 2^(M(K-2)) + ... ]
    // < K 2^(2M) [2^(M(K-1)) + 2^(M(K-2)) + ... ]
    // < K 2^(2M) 2^(M(K-1))*2 = 2^(M*K+M+k+1)
    mpn_fft_norm_mod_f(op, pl, b, pla)
}

// This is mpn_mul_fft from mpn/generic/mul_fft.c.
pub(crate) fn mpn_mul_fft(op: &mut [Limb], pl: usize, n: &[Limb], m: &[Limb], k: usize) -> Limb {
    let nl = n.len();
    let ml = m.len();
    let sqr = n as *const [Limb] == m as *const [Limb];
    assert_eq!(mpn_fft_next_size(pl, k), pl);

    let big_n = pl * Limb::WIDTH as usize;
    let mut tmp = vec![0; 2 << k];
    let mut remainder: &mut [usize] = &mut tmp;
    let mut fft_l = Vec::with_capacity(k + 1);
    for i in 0..k + 1 {
        // force remainder to move rather than be borrowed
        let (tmp_lo, tmp_hi) = { remainder }.split_at_mut(1 << i);
        fft_l.push(tmp_lo);
        remainder = tmp_hi;
    }
    mpn_fft_initl(&mut fft_l, k);
    let big_k = 1 << k;
    let big_m = big_n >> k; // N = 2^k M
    let l = 1 + (big_m - 1) / Limb::WIDTH as usize;
    let max_lk = mpn_mul_fft_lcm(Limb::WIDTH as usize, k); // lcm (GMP_NUMB_BITS, 2^k)

    let mut big_nprime = (1 + (2 * big_m + k + 2) / max_lk) * max_lk;
    // Nprime = ceil((2*M+k+3)/maxLK)*maxLK;
    let mut nprime = big_nprime / Limb::WIDTH as usize;
    // we should ensure that recursively, nprime is a multiple of the next big_k
    if nprime
        >= if sqr {
            SQR_FFT_MODF_THRESHOLD
        } else {
            MUL_FFT_MODF_THRESHOLD
        }
    {
        loop {
            let k2 = 1 << mpn_fft_best_k(nprime, sqr);
            if (nprime & (k2 - 1)) == 0 {
                break;
            }
            nprime = (nprime + k2 - 1) & k2.wrapping_neg();
            big_nprime = nprime * Limb::WIDTH as usize;
            // warning: since nprime changed, K2 may change too!
        }
    }
    assert!(nprime < pl); // otherwise we'll loop
    let mut t = vec![0; 2 * (nprime + 1)];
    let mp = big_nprime >> k;

    let mut a = vec![0; big_k * (nprime + 1)];
    let ap = mpn_mul_fft_decompose(&mut a, big_k, nprime, n, nl, l, mp, &mut t);
    let immut_fft_l: Vec<&[usize]> = fft_l.into_iter().map(|row| &*row).collect();
    if sqr {
        let pla = l * (big_k - 1) + nprime + 1; // number of required limbs for p
        let mut b = vec![0; pla];
        mpn_mul_fft_internal(
            op,
            pl,
            k,
            ap,
            &mut b,
            nprime,
            l,
            mp,
            &immut_fft_l,
            &mut t,
            sqr,
        )
    } else {
        let mut b = vec![0; big_k * (nprime + 1)];
        mpn_mul_fft_decompose(&mut b, big_k, nprime, m, ml, l, mp, &mut t);
        mpn_mul_fft_internal(
            op,
            pl,
            k,
            ap,
            &mut b,
            nprime,
            l,
            mp,
            &immut_fft_l,
            &mut t,
            sqr,
        )
    }
}