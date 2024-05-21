// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001, 2003-2022 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{
    float_either_zero, float_infinity, float_nan, float_negative_infinity, float_negative_zero,
    float_zero, Float,
};
use core::cmp::{
    max,
    Ordering::{self, *},
};
use core::mem::swap;
use core::ops::{Add, AddAssign};
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::SaturatingFrom;
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_add::{
    add_float_significands_in_place, add_float_significands_in_place_ref,
    add_float_significands_ref_ref,
};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::natural::arithmetic::float_sub::{
    sub_float_significands_in_place, sub_float_significands_in_place_ref,
    sub_float_significands_ref_ref,
};
use malachite_nz::platform::Limb;
use malachite_q::Rational;

impl Float {
    pub(crate) fn add_prec_round_assign_helper(
        &mut self,
        other: Float,
        prec: u64,
        rm: RoundingMode,
        subtract: bool,
    ) -> Ordering {
        assert_ne!(prec, 0);
        match (&mut *self, other, subtract) {
            (float_nan!(), _, _)
            | (_, float_nan!(), _)
            | (float_infinity!(), float_negative_infinity!(), false)
            | (float_negative_infinity!(), float_infinity!(), false)
            | (float_infinity!(), float_infinity!(), true)
            | (float_negative_infinity!(), float_negative_infinity!(), true) => {
                *self = float_nan!();
                Equal
            }
            (float_infinity!(), _, _)
            | (_, float_infinity!(), false)
            | (_, float_negative_infinity!(), true) => {
                *self = float_infinity!();
                Equal
            }
            (float_negative_infinity!(), _, _)
            | (_, float_negative_infinity!(), false)
            | (_, float_infinity!(), true) => {
                *self = float_negative_infinity!();
                Equal
            }
            (float_zero!(), float_negative_zero!(), false)
            | (float_negative_zero!(), float_zero!(), false)
            | (float_zero!(), float_zero!(), true)
            | (float_negative_zero!(), float_negative_zero!(), true) => {
                *self = if rm == Floor {
                    float_negative_zero!()
                } else {
                    float_zero!()
                };
                Equal
            }
            (float_either_zero!(), mut z, subtract) => {
                if subtract {
                    z.neg_assign();
                }
                let o = z.set_prec_round(prec, rm);
                *self = z;
                o
            }
            (z, float_either_zero!(), _) => z.set_prec_round(prec, rm),
            (
                Float(Finite {
                    sign: ref mut x_sign,
                    exponent: ref mut x_exp,
                    precision: ref mut x_prec,
                    significand: ref mut x,
                }),
                Float(Finite {
                    sign: mut y_sign,
                    exponent: y_exp,
                    precision: y_prec,
                    significand: mut y,
                }),
                subtract,
            ) => {
                if subtract {
                    y_sign.not_assign();
                }
                let (o, swapped) = if *x_sign == y_sign {
                    add_float_significands_in_place(
                        x,
                        x_exp,
                        *x_prec,
                        &mut y,
                        y_exp,
                        y_prec,
                        prec,
                        if *x_sign { rm } else { -rm },
                    )
                } else {
                    let (o, swapped, neg) = sub_float_significands_in_place(
                        x,
                        x_exp,
                        *x_prec,
                        &mut y,
                        y_exp,
                        y_prec,
                        prec,
                        if *x_sign { rm } else { -rm },
                    );
                    if *x == 0u32 {
                        *self = if rm == Floor {
                            float_negative_zero!()
                        } else {
                            float_zero!()
                        };
                        return o;
                    }
                    if neg {
                        x_sign.not_assign();
                    }
                    (o, swapped)
                };
                if swapped {
                    swap(x, &mut y);
                }
                *x_prec = prec;
                if *x_sign {
                    o
                } else {
                    o.reverse()
                }
            }
        }
    }

    pub(crate) fn add_prec_round_assign_ref_helper(
        &mut self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
        subtract: bool,
    ) -> Ordering {
        assert_ne!(prec, 0);
        match (&mut *self, other, subtract) {
            (x @ float_nan!(), _, _)
            | (x, float_nan!(), _)
            | (x @ float_infinity!(), float_negative_infinity!(), false)
            | (x @ float_negative_infinity!(), float_infinity!(), false)
            | (x @ float_infinity!(), float_infinity!(), true)
            | (x @ float_negative_infinity!(), float_negative_infinity!(), true) => {
                *x = float_nan!();
                Equal
            }
            (x @ float_infinity!(), _, _)
            | (x, float_infinity!(), false)
            | (x, float_negative_infinity!(), true) => {
                *x = float_infinity!();
                Equal
            }
            (x @ float_negative_infinity!(), _, _)
            | (x, float_negative_infinity!(), false)
            | (x, float_infinity!(), true) => {
                *x = float_negative_infinity!();
                Equal
            }
            (x @ float_zero!(), float_negative_zero!(), false)
            | (x @ float_negative_zero!(), float_zero!(), false)
            | (x @ float_zero!(), float_zero!(), true)
            | (x @ float_negative_zero!(), float_negative_zero!(), true) => {
                *x = if rm == Floor {
                    float_negative_zero!()
                } else {
                    float_zero!()
                };
                Equal
            }
            (x @ float_either_zero!(), z, subtract) => {
                x.clone_from(z);
                if subtract {
                    x.neg_assign();
                }
                x.set_prec_round(prec, rm)
            }
            (z, float_either_zero!(), _) => z.set_prec_round(prec, rm),
            (
                &mut Float(Finite {
                    sign: ref mut x_sign,
                    exponent: ref mut x_exp,
                    precision: ref mut x_prec,
                    significand: ref mut x,
                }),
                Float(Finite {
                    sign: mut y_sign,
                    exponent: y_exp,
                    precision: y_prec,
                    significand: y,
                }),
                subtract,
            ) => {
                if subtract {
                    y_sign.not_assign();
                }
                let o = if *x_sign == y_sign {
                    add_float_significands_in_place_ref(
                        x,
                        x_exp,
                        *x_prec,
                        y,
                        *y_exp,
                        *y_prec,
                        prec,
                        if *x_sign { rm } else { -rm },
                    )
                } else {
                    let (o, neg) = sub_float_significands_in_place_ref(
                        x,
                        x_exp,
                        *x_prec,
                        y,
                        *y_exp,
                        *y_prec,
                        prec,
                        if *x_sign { rm } else { -rm },
                    );
                    if *x == 0u32 {
                        *self = if rm == Floor {
                            float_negative_zero!()
                        } else {
                            float_zero!()
                        };
                        return o;
                    }
                    if neg {
                        x_sign.not_assign();
                    }
                    o
                };
                *x_prec = prec;
                if *x_sign {
                    o
                } else {
                    o.reverse()
                }
            }
        }
    }

    pub(crate) fn add_prec_round_ref_ref_helper(
        &self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
        subtract: bool,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        match (self, other, subtract) {
            (float_nan!(), _, _)
            | (_, float_nan!(), _)
            | (float_infinity!(), float_negative_infinity!(), false)
            | (float_negative_infinity!(), float_infinity!(), false)
            | (float_infinity!(), float_infinity!(), true)
            | (float_negative_infinity!(), float_negative_infinity!(), true) => {
                (float_nan!(), Equal)
            }
            (float_infinity!(), _, _)
            | (_, float_infinity!(), false)
            | (_, float_negative_infinity!(), true) => (float_infinity!(), Equal),
            (float_negative_infinity!(), _, _)
            | (_, float_negative_infinity!(), false)
            | (_, float_infinity!(), true) => (float_negative_infinity!(), Equal),
            (float_zero!(), float_negative_zero!(), false)
            | (float_negative_zero!(), float_zero!(), false)
            | (float_zero!(), float_zero!(), true)
            | (float_negative_zero!(), float_negative_zero!(), true) => (
                if rm == Floor {
                    float_negative_zero!()
                } else {
                    float_zero!()
                },
                Equal,
            ),
            (float_either_zero!(), z, subtract) => {
                let mut out = if subtract { -z } else { z.clone() };
                let o = out.set_prec_round(prec, rm);
                (out, o)
            }
            (z, float_either_zero!(), _) => {
                let mut out = z.clone();
                let o = out.set_prec_round(prec, rm);
                (out, o)
            }
            (
                Float(Finite {
                    sign: x_sign,
                    exponent: x_exp,
                    precision: x_prec,
                    significand: x,
                }),
                Float(Finite {
                    sign: mut y_sign,
                    exponent: y_exp,
                    precision: y_prec,
                    significand: y,
                }),
                subtract,
            ) => {
                if subtract {
                    y_sign.not_assign();
                }
                if *x_sign == y_sign {
                    let (sum, sum_exp, o) = add_float_significands_ref_ref(
                        x,
                        *x_exp,
                        *x_prec,
                        y,
                        *y_exp,
                        *y_prec,
                        prec,
                        if *x_sign { rm } else { -rm },
                    );
                    let sum = Float(Finite {
                        sign: *x_sign,
                        exponent: sum_exp,
                        precision: prec,
                        significand: sum,
                    });
                    (sum, if *x_sign { o } else { o.reverse() })
                } else {
                    let (diff, diff_exp, o, neg) = sub_float_significands_ref_ref(
                        x,
                        *x_exp,
                        *x_prec,
                        y,
                        *y_exp,
                        *y_prec,
                        prec,
                        if *x_sign { rm } else { -rm },
                    );
                    if diff == 0u32 {
                        (
                            if rm == Floor {
                                float_negative_zero!()
                            } else {
                                float_zero!()
                            },
                            o,
                        )
                    } else {
                        let diff = Float(Finite {
                            sign: *x_sign != neg,
                            exponent: diff_exp,
                            precision: prec,
                            significand: diff,
                        });
                        (diff, if *x_sign != neg { o } else { o.reverse() })
                    }
                }
            }
        }
    }

    // This is mpfr_add_q from gmp_op.c, MPFR 4.2.0.
    pub(crate) fn add_rational_prec_round_assign_helper(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        assert_ne!(prec, 0);
        match (&mut *self, other) {
            (float_nan!() | float_infinity!() | float_negative_infinity!(), _) => Equal,
            (float_negative_zero!(), y) => {
                if y == 0u32 {
                    Equal
                } else {
                    let o;
                    (*self, o) = Float::from_rational_prec_round(y, prec, rm);
                    o
                }
            }
            (float_zero!(), y) => {
                let o;
                (*self, o) = Float::from_rational_prec_round(y, prec, rm);
                o
            }
            (_, y) if y == 0 => self.set_prec_round(prec, rm),
            (x, y) => {
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                loop {
                    // Error <= 1/2 ulp(q)
                    let (q, o) = Float::from_rational_prec_ref(&y, working_prec);
                    if o == Equal {
                        // Result is exact so we can add it directly!
                        return self.add_prec_round_assign(q, prec, rm);
                    }
                    let q_exp = q.get_exponent().unwrap();
                    let t = x.add_prec_ref_val(q, working_prec).0;
                    // Error on t is <= 1/2 ulp(t).
                    // ```
                    // Error / ulp(t)      <= 1/2 + 1/2 * 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)>0, <= 2^(EXP(q)-EXP(t)-1)*(1+2^-(EXP(q)-EXP(t)))
                    //                     <= 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)<0, <= 2^0
                    // ```
                    // We can get 0, but we can't round since q is inexact
                    if t != 0 {
                        let m = u64::saturating_from(q_exp - t.get_exponent().unwrap())
                            .checked_add(1)
                            .unwrap();
                        if working_prec >= m
                            && float_can_round(
                                t.significand_ref().unwrap(),
                                working_prec - m,
                                prec,
                                rm,
                            )
                        {
                            *self = t;
                            return self.set_prec_round(prec, rm);
                        }
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    pub(crate) fn add_rational_prec_round_assign_ref_helper(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        assert_ne!(prec, 0);
        match (&mut *self, other) {
            (float_nan!() | float_infinity!() | float_negative_infinity!(), _) => Equal,
            (float_negative_zero!(), y) => {
                if *y == 0u32 {
                    Equal
                } else {
                    let o;
                    (*self, o) = Float::from_rational_prec_round_ref(y, prec, rm);
                    o
                }
            }
            (float_zero!(), y) => {
                let o;
                (*self, o) = Float::from_rational_prec_round_ref(y, prec, rm);
                o
            }
            (_, y) if *y == 0 => self.set_prec_round(prec, rm),
            (x, y) => {
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                // working_prec grows as O([(1 + sqrt(3)) / 2] ^ n) ≈ O(1.366 ^ n).
                loop {
                    // Error <= 1/2 ulp(q)
                    let (q, o) = Float::from_rational_prec_ref(y, working_prec);
                    if o == Equal {
                        // Result is exact so we can add it directly!
                        return self.add_prec_round_assign(q, prec, rm);
                    }
                    let q_exp = q.get_exponent().unwrap();
                    let t = x.add_prec_ref_val(q, working_prec).0;
                    // Error on t is <= 1/2 ulp(t).
                    // ```
                    // Error / ulp(t)      <= 1/2 + 1/2 * 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)>0, <= 2^(EXP(q)-EXP(t)-1)*(1+2^-(EXP(q)-EXP(t)))
                    //                     <= 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)<0, <= 2^0
                    // ```
                    // We can get 0, but we can't round since q is inexact
                    if t != 0 {
                        let m = u64::saturating_from(q_exp - t.get_exponent().unwrap())
                            .checked_add(1)
                            .unwrap();
                        if working_prec >= m
                            && float_can_round(
                                t.significand_ref().unwrap(),
                                working_prec - m,
                                prec,
                                rm,
                            )
                        {
                            *self = t;
                            return self.set_prec_round(prec, rm);
                        }
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    pub(crate) fn add_rational_prec_round_ref_val_helper(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        match (self, other) {
            (float_nan!(), _) => (float_nan!(), Equal),
            (float_infinity!(), _) => (float_infinity!(), Equal),
            (float_negative_infinity!(), _) => (float_negative_infinity!(), Equal),
            (float_negative_zero!(), y) => {
                if y == 0u32 {
                    (float_negative_zero!(), Equal)
                } else {
                    Float::from_rational_prec_round(y, prec, rm)
                }
            }
            (float_zero!(), y) => Float::from_rational_prec_round(y, prec, rm),
            (_, y) if y == 0 => {
                let mut x = self.clone();
                let o = x.set_prec_round(prec, rm);
                (x, o)
            }
            (x, y) => {
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                // working_prec grows as O([(1 + sqrt(3)) / 2] ^ n) ≈ O(1.366 ^ n).
                loop {
                    // Error <= 1/2 ulp(q)
                    let (q, o) = Float::from_rational_prec_ref(&y, working_prec);
                    if o == Equal {
                        // Result is exact so we can add it directly!
                        let mut x = self.clone();
                        let o = x.add_prec_round_assign(q, prec, rm);
                        return (x, o);
                    }
                    let q_exp = q.get_exponent().unwrap();
                    let mut t = x.add_prec_ref_val(q, working_prec).0;
                    // Error on t is <= 1/2 ulp(t).
                    // ```
                    // Error / ulp(t)      <= 1/2 + 1/2 * 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)>0, <= 2^(EXP(q)-EXP(t)-1)*(1+2^-(EXP(q)-EXP(t)))
                    //                     <= 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)<0, <= 2^0
                    // ```
                    // We can get 0, but we can't round since q is inexact
                    if t != 0 {
                        let m = u64::saturating_from(q_exp - t.get_exponent().unwrap())
                            .checked_add(1)
                            .unwrap();
                        if working_prec >= m
                            && float_can_round(
                                t.significand_ref().unwrap(),
                                working_prec - m,
                                prec,
                                rm,
                            )
                        {
                            let o = t.set_prec_round(prec, rm);
                            return (t, o);
                        }
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    pub(crate) fn add_rational_prec_round_ref_ref_helper(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        match (self, other) {
            (float_nan!(), _) => (float_nan!(), Equal),
            (float_infinity!(), _) => (float_infinity!(), Equal),
            (float_negative_infinity!(), _) => (float_negative_infinity!(), Equal),
            (float_negative_zero!(), y) => {
                if *y == 0u32 {
                    (float_negative_zero!(), Equal)
                } else {
                    Float::from_rational_prec_round_ref(y, prec, rm)
                }
            }
            (float_zero!(), y) => Float::from_rational_prec_round_ref(y, prec, rm),
            (_, y) if *y == 0 => {
                let mut x = self.clone();
                let o = x.set_prec_round(prec, rm);
                (x, o)
            }
            (x, y) => {
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                // working_prec grows as O([(1 + sqrt(3)) / 2] ^ n) ≈ O(1.366 ^ n).
                loop {
                    // Error <= 1/2 ulp(q)
                    let (q, o) = Float::from_rational_prec_ref(y, working_prec);
                    if o == Equal {
                        // Result is exact so we can add it directly!
                        let mut x = self.clone();
                        let o = x.add_prec_round_assign(q, prec, rm);
                        return (x, o);
                    }
                    let q_exp = q.get_exponent().unwrap();
                    let mut t = x.add_prec_ref_val(q, working_prec).0;
                    // Error on t is <= 1/2 ulp(t).
                    // ```
                    // Error / ulp(t)      <= 1/2 + 1/2 * 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)>0, <= 2^(EXP(q)-EXP(t)-1)*(1+2^-(EXP(q)-EXP(t)))
                    //                     <= 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)<0, <= 2^0
                    // ```
                    // We can get 0, but we can't round since q is inexact
                    if t != 0 {
                        let m = u64::saturating_from(q_exp - t.get_exponent().unwrap())
                            .checked_add(1)
                            .unwrap();
                        if working_prec >= m
                            && float_can_round(
                                t.significand_ref().unwrap(),
                                working_prec - m,
                                prec,
                                rm,
                            )
                        {
                            let o = t.set_prec_round(prec, rm);
                            return (t, o);
                        }
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    // This is mpfr_sub_q from gmp_op.c, MPFR 4.2.0.
    pub(crate) fn sub_rational_prec_round_assign_helper(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        assert_ne!(prec, 0);
        match (&mut *self, other) {
            (float_nan!() | float_infinity!() | float_negative_infinity!(), _) => Equal,
            (float_negative_zero!(), y) => {
                let o;
                (*self, o) = Float::from_rational_prec_round(y, prec, -rm);
                self.neg_assign();
                o.reverse()
            }
            (float_zero!(), y) => {
                if y == 0u32 {
                    Equal
                } else {
                    let o;
                    (*self, o) = Float::from_rational_prec_round(y, prec, -rm);
                    self.neg_assign();
                    o.reverse()
                }
            }
            (_, y) if y == 0 => self.set_prec_round(prec, rm),
            (x, y) => {
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                // working_prec grows as O([(1 + sqrt(3)) / 2] ^ n) ≈ O(1.366 ^ n).
                loop {
                    // Error <= 1/2 ulp(q)
                    let (q, o) = Float::from_rational_prec_ref(&y, working_prec);
                    if o == Equal {
                        // Result is exact so we can add it directly!
                        return self.sub_prec_round_assign(q, prec, rm);
                    }
                    let q_exp = q.get_exponent().unwrap();
                    let t = x.sub_prec_ref_val(q, working_prec).0;
                    // Error on t is <= 1/2 ulp(t).
                    // ```
                    // Error / ulp(t)      <= 1/2 + 1/2 * 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)>0, <= 2^(EXP(q)-EXP(t)-1)*(1+2^-(EXP(q)-EXP(t)))
                    //                     <= 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)<0, <= 2^0
                    // ```
                    // We can get 0, but we can't round since q is inexact
                    if t != 0 {
                        let m = u64::saturating_from(q_exp - t.get_exponent().unwrap())
                            .checked_add(1)
                            .unwrap();
                        if working_prec >= m
                            && float_can_round(
                                t.significand_ref().unwrap(),
                                working_prec - m,
                                prec,
                                rm,
                            )
                        {
                            *self = t;
                            return self.set_prec_round(prec, rm);
                        }
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    pub(crate) fn sub_rational_prec_round_assign_ref_helper(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        assert_ne!(prec, 0);
        match (&mut *self, other) {
            (float_nan!() | float_infinity!() | float_negative_infinity!(), _) => Equal,
            (float_negative_zero!(), y) => {
                let o;
                (*self, o) = Float::from_rational_prec_round_ref(y, prec, -rm);
                self.neg_assign();
                o.reverse()
            }
            (float_zero!(), y) => {
                if *y == 0u32 {
                    Equal
                } else {
                    let o;
                    (*self, o) = Float::from_rational_prec_round_ref(y, prec, -rm);
                    self.neg_assign();
                    o.reverse()
                }
            }
            (_, y) if *y == 0 => self.set_prec_round(prec, rm),
            (x, y) => {
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                // working_prec grows as O([(1 + sqrt(3)) / 2] ^ n) ≈ O(1.366 ^ n).
                loop {
                    // Error <= 1/2 ulp(q)
                    let (q, o) = Float::from_rational_prec_ref(y, working_prec);
                    if o == Equal {
                        // Result is exact so we can add it directly!
                        return self.sub_prec_round_assign(q, prec, rm);
                    }
                    let q_exp = q.get_exponent().unwrap();
                    let t = x.sub_prec_ref_val(q, working_prec).0;
                    // Error on t is <= 1/2 ulp(t).
                    // ```
                    // Error / ulp(t)      <= 1/2 + 1/2 * 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)>0, <= 2^(EXP(q)-EXP(t)-1)*(1+2^-(EXP(q)-EXP(t)))
                    //                     <= 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)<0, <= 2^0
                    // ```
                    // We can get 0, but we can't round since q is inexact
                    if t != 0 {
                        let m = u64::saturating_from(q_exp - t.get_exponent().unwrap())
                            .checked_add(1)
                            .unwrap();
                        if working_prec >= m
                            && float_can_round(
                                t.significand_ref().unwrap(),
                                working_prec - m,
                                prec,
                                rm,
                            )
                        {
                            *self = t;
                            return self.set_prec_round(prec, rm);
                        }
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    pub(crate) fn sub_rational_prec_round_ref_val_helper(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        match (self, other) {
            (float_nan!(), _) => (float_nan!(), Equal),
            (float_infinity!(), _) => (float_infinity!(), Equal),
            (float_negative_infinity!(), _) => (float_negative_infinity!(), Equal),
            (float_negative_zero!(), y) => {
                let (diff, o) = Float::from_rational_prec_round(y, prec, -rm);
                (-diff, o.reverse())
            }
            (float_zero!(), y) => {
                if y == 0u32 {
                    (float_zero!(), Equal)
                } else {
                    let (diff, o) = Float::from_rational_prec_round(y, prec, -rm);
                    (-diff, o.reverse())
                }
            }
            (_, y) if y == 0 => {
                let mut x = self.clone();
                let o = x.set_prec_round(prec, rm);
                (x, o)
            }
            (x, y) => {
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                // working_prec grows as O([(1 + sqrt(3)) / 2] ^ n) ≈ O(1.366 ^ n).
                loop {
                    // Error <= 1/2 ulp(q)
                    let (q, o) = Float::from_rational_prec_ref(&y, working_prec);
                    if o == Equal {
                        // Result is exact so we can add it directly!
                        let mut x = self.clone();
                        let o = x.sub_prec_round_assign(q, prec, rm);
                        return (x, o);
                    }
                    let q_exp = q.get_exponent().unwrap();
                    let mut t = x.sub_prec_ref_val(q, working_prec).0;
                    // Error on t is <= 1/2 ulp(t).
                    // ```
                    // Error / ulp(t)      <= 1/2 + 1/2 * 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)>0, <= 2^(EXP(q)-EXP(t)-1)*(1+2^-(EXP(q)-EXP(t)))
                    //                     <= 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)<0, <= 2^0
                    // ```
                    // We can get 0, but we can't round since q is inexact
                    if t != 0 {
                        let m = u64::saturating_from(q_exp - t.get_exponent().unwrap())
                            .checked_add(1)
                            .unwrap();
                        if working_prec >= m
                            && float_can_round(
                                t.significand_ref().unwrap(),
                                working_prec - m,
                                prec,
                                rm,
                            )
                        {
                            let o = t.set_prec_round(prec, rm);
                            return (t, o);
                        }
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    pub(crate) fn sub_rational_prec_round_ref_ref_helper(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        match (self, other) {
            (float_nan!(), _) => (float_nan!(), Equal),
            (float_infinity!(), _) => (float_infinity!(), Equal),
            (float_negative_infinity!(), _) => (float_negative_infinity!(), Equal),
            (float_negative_zero!(), y) => {
                let (diff, o) = Float::from_rational_prec_round_ref(y, prec, -rm);
                (-diff, o.reverse())
            }
            (float_zero!(), y) => {
                if *y == 0u32 {
                    (float_zero!(), Equal)
                } else {
                    let (diff, o) = Float::from_rational_prec_round_ref(y, prec, -rm);
                    (-diff, o.reverse())
                }
            }
            (_, y) if *y == 0 => {
                let mut x = self.clone();
                let o = x.set_prec_round(prec, rm);
                (x, o)
            }
            (x, y) => {
                let mut working_prec = prec + 10;
                let mut increment = Limb::WIDTH;
                // working_prec grows as O([(1 + sqrt(3)) / 2] ^ n) ≈ O(1.366 ^ n).
                loop {
                    // Error <= 1/2 ulp(q)
                    let (q, o) = Float::from_rational_prec_ref(y, working_prec);
                    if o == Equal {
                        // Result is exact so we can add it directly!
                        let mut x = self.clone();
                        let o = x.sub_prec_round_assign(q, prec, rm);
                        return (x, o);
                    }
                    let q_exp = q.get_exponent().unwrap();
                    let mut t = x.sub_prec_ref_val(q, working_prec).0;
                    // Error on t is <= 1/2 ulp(t).
                    // ```
                    // Error / ulp(t)      <= 1/2 + 1/2 * 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)>0, <= 2^(EXP(q)-EXP(t)-1)*(1+2^-(EXP(q)-EXP(t)))
                    //                     <= 2^(EXP(q)-EXP(t))
                    // If EXP(q)-EXP(t)<0, <= 2^0
                    // ```
                    // We can get 0, but we can't round since q is inexact
                    if t != 0 {
                        let m = u64::saturating_from(q_exp - t.get_exponent().unwrap())
                            .checked_add(1)
                            .unwrap();
                        if working_prec >= m
                            && float_can_round(
                                t.significand_ref().unwrap(),
                                working_prec - m,
                                prec,
                                rm,
                            )
                        {
                            let o = t.set_prec_round(prec, rm);
                            return (t, o);
                        }
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
            }
        }
    }

    /// Adds two [`Float`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. Both [`Float`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any other [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0,p,m)=0.0$
    /// - $f(-0.0,-0.0,p,m)=-0.0$
    /// - $f(0.0,-0.0,p,m)=f(-0.0,0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,-0.0,p,m)=f(-0.0,0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,-x,p,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,-x,p,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::add_round`] instead. If both of these things are true, consider using `+`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round(Float::from(E), 5, Floor);
    /// assert_eq!(sum.to_string(), "5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round(Float::from(E), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "6.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round(Float::from(E), 5, Nearest);
    /// assert_eq!(sum.to_string(), "5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round(Float::from(E), 20, Floor);
    /// assert_eq!(sum.to_string(), "5.85987");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round(Float::from(E), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "5.85988");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round(Float::from(E), 20, Nearest);
    /// assert_eq!(sum.to_string(), "5.85987");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_round(
        mut self,
        other: Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.add_prec_round_assign(other, prec, rm);
        (self, o)
    }

    /// Adds two [`Float`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. The first [`Float`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0,p,m)=0.0$
    /// - $f(-0.0,-0.0,p,m)=-0.0$
    /// - $f(0.0,-0.0,p,m)=f(-0.0,0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,-0.0,p,m)=f(-0.0,0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,-x,p,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,-x,p,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_prec_val_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::add_round_val_ref`] instead. If both of these things are true,
    /// consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_val_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(sum.to_string(), "5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_val_ref(&Float::from(E), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "6.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_val_ref(&Float::from(E), 5, Nearest);
    /// assert_eq!(sum.to_string(), "5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_val_ref(&Float::from(E), 20, Floor);
    /// assert_eq!(sum.to_string(), "5.85987");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_val_ref(&Float::from(E), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "5.85988");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_val_ref(&Float::from(E), 20, Nearest);
    /// assert_eq!(sum.to_string(), "5.85987");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_round_val_ref(
        mut self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.add_prec_round_assign_ref(other, prec, rm);
        (self, o)
    }

    /// Adds two [`Float`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. The first [`Float`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0,p,m)=0.0$
    /// - $f(-0.0,-0.0,p,m)=-0.0$
    /// - $f(0.0,-0.0,p,m)=f(-0.0,0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,-0.0,p,m)=f(-0.0,0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,-x,p,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,-x,p,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_prec_ref_val`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::add_round_ref_val`] instead. If both of these things are true,
    /// consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_val_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(sum.to_string(), "5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_ref_val(Float::from(E), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "6.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_ref_val(Float::from(E), 5, Nearest);
    /// assert_eq!(sum.to_string(), "5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_ref_val(Float::from(E), 20, Floor);
    /// assert_eq!(sum.to_string(), "5.85987");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_ref_val(Float::from(E), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "5.85988");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_ref_val(Float::from(E), 20, Nearest);
    /// assert_eq!(sum.to_string(), "5.85987");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_round_ref_val(
        &self,
        mut other: Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = other.add_prec_round_assign_ref(self, prec, rm);
        (other, o)
    }

    /// Adds two [`Float`]s, rounding the result to the specified precision and with the specified
    /// rounding mode. Both [`Float`]s are taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any other [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0,p,m)=0.0$
    /// - $f(-0.0,-0.0,p,m)=-0.0$
    /// - $f(0.0,-0.0,p,m)=f(-0.0,0.0,p,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,-0.0,p,m)=f(-0.0,0.0,p,m)=-0.0$ if $m$ is `Floor`
    /// - $f(x,-x,p,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,-x,p,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_prec_ref_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::add_round_ref_ref`] instead. If both of these things are true,
    /// consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_ref_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(sum.to_string(), "5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_ref_ref(&Float::from(E), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "6.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_ref_ref(&Float::from(E), 5, Nearest);
    /// assert_eq!(sum.to_string(), "5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_ref_ref(&Float::from(E), 20, Floor);
    /// assert_eq!(sum.to_string(), "5.85987");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_ref_ref(&Float::from(E), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "5.85988");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_round_ref_ref(&Float::from(E), 20, Nearest);
    /// assert_eq!(sum.to_string(), "5.85987");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_round_ref_ref(
        &self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        self.add_prec_round_ref_ref_helper(other, prec, rm, false)
    }

    /// Adds two [`Float`]s, rounding the result to the nearest value of the specified precision.
    /// Both [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any other [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,-\infty,p)=f(-\infty,\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0,p)=0.0$
    /// - $f(-0.0,-0.0,p)=-0.0$
    /// - $f(0.0,-0.0,p)=f(-0.0,0.0,p)=0.0$
    /// - $f(x,-x,p)=0.0$ if $x$ is finite and nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_prec(Float::from(E), 5);
    /// assert_eq!(sum.to_string(), "5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec(Float::from(E), 20);
    /// assert_eq!(sum.to_string(), "5.85987");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec(self, other: Float, prec: u64) -> (Float, Ordering) {
        self.add_prec_round(other, prec, Nearest)
    }

    /// Adds two [`Float`]s, rounding the result to the nearest value of the specified precision.
    /// The first [`Float`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any other [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,-\infty,p)=f(-\infty,\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0,p)=0.0$
    /// - $f(-0.0,-0.0,p)=-0.0$
    /// - $f(0.0,-0.0,p)=f(-0.0,0.0,p)=0.0$
    /// - $f(x,-x,p)=0.0$ if $x$ is finite and nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_prec_round_val_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_prec_val_ref(&Float::from(E), 5);
    /// assert_eq!(sum.to_string(), "5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_prec_val_ref(&Float::from(E), 20);
    /// assert_eq!(sum.to_string(), "5.85987");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_val_ref(self, other: &Float, prec: u64) -> (Float, Ordering) {
        self.add_prec_round_val_ref(other, prec, Nearest)
    }

    /// Adds two [`Float`]s, rounding the result to the nearest value of the specified precision.
    /// The first [`Float`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any other [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,-\infty,p)=f(-\infty,\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0,p)=0.0$
    /// - $f(-0.0,-0.0,p)=-0.0$
    /// - $f(0.0,-0.0,p)=f(-0.0,0.0,p)=0.0$
    /// - $f(x,-x,p)=0.0$ if $x$ is finite and nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_prec_round_ref_val`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = (&Float::from(PI)).add_prec_ref_val(Float::from(E), 5);
    /// assert_eq!(sum.to_string(), "5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = (&Float::from(PI)).add_prec_ref_val(Float::from(E), 20);
    /// assert_eq!(sum.to_string(), "5.85987");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_ref_val(&self, other: Float, prec: u64) -> (Float, Ordering) {
        self.add_prec_round_ref_val(other, prec, Nearest)
    }

    /// Adds two [`Float`]s, rounding the result to the nearest value of the specified precision.
    /// Both [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are
    /// not comparable to any other [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,-\infty,p)=f(-\infty,\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0,p)=0.0$
    /// - $f(-0.0,-0.0,p)=-0.0$
    /// - $f(0.0,-0.0,p)=f(-0.0,0.0,p)=0.0$
    /// - $f(x,-x,p)=0.0$ if $x$ is finite and nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_prec_round_ref_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = (&Float::from(PI)).add_prec_ref_ref(&Float::from(E), 5);
    /// assert_eq!(sum.to_string(), "5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = (&Float::from(PI)).add_prec_ref_ref(&Float::from(E), 20);
    /// assert_eq!(sum.to_string(), "5.85987");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_ref_ref(&self, other: &Float, prec: u64) -> (Float, Ordering) {
        self.add_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Adds two [`Float`]s, rounding the result with the specified rounding mode. Both [`Float`]s
    /// are taken by value. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any other [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\infty,-\infty,m)=f(-\infty,\infty,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,m)=f(x,\infty,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=f(x,-\infty,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0,m)=0.0$
    /// - $f(-0.0,-0.0,m)=-0.0$
    /// - $f(0.0,-0.0,m)=f(-0.0,0.0,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,-0.0,m)=f(-0.0,0.0,m)=-0.0$ if $m$ is `Floor`
    /// - $f(0.0,x,m)=f(x,0.0,m)=f(-0.0,x,m)=f(x,-0.0,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,-x,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,-x,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using [`Float::add_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `+`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_round(Float::from(E), Floor);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_round(Float::from(E), Ceiling);
    /// assert_eq!(sum.to_string(), "5.859874482048839");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_round(Float::from(E), Nearest);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_round(self, other: Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.add_prec_round(other, prec, rm)
    }

    /// Adds two [`Float`]s, rounding the result with the specified rounding mode. The first
    /// [`Float`] is taken by value and the second by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any other [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\infty,-\infty,m)=f(-\infty,\infty,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,m)=f(x,\infty,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=f(x,-\infty,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0,m)=0.0$
    /// - $f(-0.0,-0.0,m)=-0.0$
    /// - $f(0.0,-0.0,m)=f(-0.0,0.0,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,-0.0,m)=f(-0.0,0.0,m)=-0.0$ if $m$ is `Floor`
    /// - $f(0.0,x,m)=f(x,0.0,m)=f(-0.0,x,m)=f(x,-0.0,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,-x,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,-x,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using [`Float::add_prec_round_val_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `+`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_round_val_ref(&Float::from(E), Floor);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_round_val_ref(&Float::from(E), Ceiling);
    /// assert_eq!(sum.to_string(), "5.859874482048839");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_round_val_ref(&Float::from(E), Nearest);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_round_val_ref(self, other: &Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.add_prec_round_val_ref(other, prec, rm)
    }

    /// Adds two [`Float`]s, rounding the result with the specified rounding mode. The first
    /// [`Float`] is taken by reference and the second by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any other [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\infty,-\infty,m)=f(-\infty,\infty,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,m)=f(x,\infty,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=f(x,-\infty,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0,m)=0.0$
    /// - $f(-0.0,-0.0,m)=-0.0$
    /// - $f(0.0,-0.0,m)=f(-0.0,0.0,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,-0.0,m)=f(-0.0,0.0,m)=-0.0$ if $m$ is `Floor`
    /// - $f(0.0,x,m)=f(x,0.0,m)=f(-0.0,x,m)=f(x,-0.0,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,-x,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,-x,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using [`Float::add_prec_round_ref_val`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `+`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = (&Float::from(PI)).add_round_ref_val(Float::from(E), Floor);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = (&Float::from(PI)).add_round_ref_val(Float::from(E), Ceiling);
    /// assert_eq!(sum.to_string(), "5.859874482048839");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = (&Float::from(PI)).add_round_ref_val(Float::from(E), Nearest);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_round_ref_val(&self, other: Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.add_prec_round_ref_val(other, prec, rm)
    }

    /// Adds two [`Float`]s, rounding the result with the specified rounding mode. Both [`Float`]s
    /// are taken by reference. An [`Ordering`] is also returned, indicating whether the rounded sum
    /// is less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any other [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\infty,-\infty,m)=f(-\infty,\infty,m)=
    ///     \text{NaN}$
    /// - $f(\infty,x,m)=f(x,\infty,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=f(x,-\infty,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0,m)=0.0$
    /// - $f(-0.0,-0.0,m)=-0.0$
    /// - $f(0.0,-0.0,m)=f(-0.0,0.0,m)=0.0$ if $m$ is not `Floor`
    /// - $f(0.0,-0.0,m)=f(-0.0,0.0,m)=-0.0$ if $m$ is `Floor`
    /// - $f(0.0,x,m)=f(x,0.0,m)=f(-0.0,x,m)=f(x,-0.0,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,-x,m)=0.0$ if $x$ is finite and nonzero and $m$ is not `Floor`
    /// - $f(x,-x,m)=-0.0$ if $x$ is finite and nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using [`Float::add_prec_round_ref_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `+`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_round_ref_ref(&Float::from(E), Floor);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_round_ref_ref(&Float::from(E), Ceiling);
    /// assert_eq!(sum.to_string(), "5.859874482048839");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_round_ref_ref(&Float::from(E), Nearest);
    /// assert_eq!(sum.to_string(), "5.859874482048838");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_round_ref_ref(&self, other: &Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.add_prec_round_ref_ref(other, prec, rm)
    }

    /// Adds a [`Float`] to a [`Float`] in place, rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] on the right-hand side is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::add_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_prec_assign`] instead. If
    /// you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::add_round_assign`] instead. If both of these things are true,
    /// consider using `+=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_prec_round_assign(Float::from(E), 5, Floor), Less);
    /// assert_eq!(x.to_string(), "5.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_prec_round_assign(Float::from(E), 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "6.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_prec_round_assign(Float::from(E), 5, Nearest), Less);
    /// assert_eq!(x.to_string(), "5.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_prec_round_assign(Float::from(E), 20, Floor), Less);
    /// assert_eq!(x.to_string(), "5.85987");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign(Float::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "5.85988");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_prec_round_assign(Float::from(E), 20, Nearest), Less);
    /// assert_eq!(x.to_string(), "5.85987");
    /// ```
    #[inline]
    pub fn add_prec_round_assign(&mut self, other: Float, prec: u64, rm: RoundingMode) -> Ordering {
        self.add_prec_round_assign_helper(other, prec, rm, false)
    }

    /// Adds a [`Float`] to a [`Float`] in place, rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] on the right-hand side is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any other
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::add_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_prec_assign_ref`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::add_round_assign_ref`] instead. If both of these things are
    /// true, consider using `+=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_prec_round_assign_ref(&Float::from(E), 5, Floor), Less);
    /// assert_eq!(x.to_string(), "5.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign_ref(&Float::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "6.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign_ref(&Float::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "5.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign_ref(&Float::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "5.85987");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign_ref(&Float::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "5.85988");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign_ref(&Float::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "5.85987");
    /// ```
    #[inline]
    pub fn add_prec_round_assign_ref(
        &mut self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        self.add_prec_round_assign_ref_helper(other, prec, rm, false)
    }

    /// Adds a [`Float`] to a [`Float`] in place, rounding the result to the nearest value of the
    /// specified precision. The [`Float`] on the right-hand side is taken by value. An [`Ordering`]
    /// is returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any other [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::add_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `+=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_prec_assign(Float::from(E), 5), Less);
    /// assert_eq!(x.to_string(), "5.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_prec_assign(Float::from(E), 20), Less);
    /// assert_eq!(x.to_string(), "5.85987");
    /// ```
    #[inline]
    pub fn add_prec_assign(&mut self, other: Float, prec: u64) -> Ordering {
        self.add_prec_round_assign(other, prec, Nearest)
    }

    /// Adds a [`Float`] to a [`Float`] in place, rounding the result to the nearest value of the
    /// specified precision. The [`Float`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::add_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_prec_round_assign_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `+=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_prec_assign_ref(&Float::from(E), 5), Less);
    /// assert_eq!(x.to_string(), "5.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_prec_assign_ref(&Float::from(E), 20), Less);
    /// assert_eq!(x.to_string(), "5.85987");
    /// ```
    #[inline]
    pub fn add_prec_assign_ref(&mut self, other: &Float, prec: u64) -> Ordering {
        self.add_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Adds a [`Float`] to a [`Float`] in place, rounding the result with the specified rounding
    /// mode. The [`Float`] on the right-hand side is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any other [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::add_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using [`Float::add_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `+=`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_round_assign(Float::from(E), Floor), Less);
    /// assert_eq!(x.to_string(), "5.859874482048838");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_round_assign(Float::from(E), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "5.859874482048839");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_round_assign(Float::from(E), Nearest), Less);
    /// assert_eq!(x.to_string(), "5.859874482048838");
    /// ```
    #[inline]
    pub fn add_round_assign(&mut self, other: Float, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.add_prec_round_assign(other, prec, rm)
    }

    /// Adds a [`Float`] to a [`Float`] in place, rounding the result with the specified rounding
    /// mode. The [`Float`] on the right-hand side is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any other [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::add_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::add_prec_round_assign_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `+=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_round_assign_ref(&Float::from(E), Floor), Less);
    /// assert_eq!(x.to_string(), "5.859874482048838");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_round_assign_ref(&Float::from(E), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "5.859874482048839");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_round_assign_ref(&Float::from(E), Nearest), Less);
    /// assert_eq!(x.to_string(), "5.859874482048838");
    /// ```
    #[inline]
    pub fn add_round_assign_ref(&mut self, other: &Float, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.add_prec_round_assign_ref(other, prec, rm)
    }

    /// Adds a [`Float`] and a [`Rational`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`] and the [`Rational`] are both taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0.0,0,p,m)=0.0$
    /// - $f(-0.0,0,p,m)=-0.0$
    /// - $f(x,-x,p,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,-x,p,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_rational_prec`] instead.
    /// If you know that your target precision is the precision of the [`Float`] input, consider
    /// using [`Float::add_rational_round`] instead. If both of these things are true, consider
    /// using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(sum.to_string(), "3.4");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Nearest);
    /// assert_eq!(sum.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(sum.to_string(), "3.474922");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "3.474926");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Nearest);
    /// assert_eq!(sum.to_string(), "3.474926");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_round(
        mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.add_rational_prec_round_assign(other, prec, rm);
        (self, o)
    }

    /// Adds a [`Float`] and a [`Rational`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`] is taken by value and the [`Rational`] by
    /// reference. An  [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// other [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0.0,0,p,m)=0.0$
    /// - $f(-0.0,0,p,m)=-0.0$
    /// - $f(x,-x,p,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,-x,p,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_rational_prec_val_ref`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::add_rational_round_val_ref`] instead. If both of these things are
    /// true, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(sum.to_string(), "3.4");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(sum.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(sum.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(sum.to_string(), "3.474922");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(sum.to_string(), "3.474926");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(sum.to_string(), "3.474926");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_round_val_ref(
        mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.add_rational_prec_round_assign_ref(other, prec, rm);
        (self, o)
    }

    /// Adds a [`Float`] and a [`Rational`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`] is taken by reference and the [`Rational`] by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any other
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0.0,0,p,m)=0.0$
    /// - $f(-0.0,0,p,m)=-0.0$
    /// - $f(x,-x,p,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,-x,p,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_rational_prec_ref_val`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::add_rational_round_ref_val`] instead. If both of these things are
    /// true, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(sum.to_string(), "3.4");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(sum.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(sum.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(sum.to_string(), "3.474922");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(sum.to_string(), "3.474926");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(sum.to_string(), "3.474926");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        self.add_rational_prec_round_ref_val_helper(other, prec, rm)
    }

    /// Adds a [`Float`] and a [`Rational`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`] and the [`Rational`] are both taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to,
    /// or greater than the exact sum. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$
    /// - $f(-\infty,x,p,m)=-\infty$
    /// - $f(0.0,0,p,m)=0.0$
    /// - $f(-0.0,0,p,m)=-0.0$
    /// - $f(x,-x,p,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,-x,p,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_rational_prec_ref_ref`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::add_rational_round_ref_ref`] instead. If both of these things are
    /// true, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(sum.to_string(), "3.4");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(sum.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(sum.to_string(), "3.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(sum.to_string(), "3.474922");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(sum.to_string(), "3.474926");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(sum.to_string(), "3.474926");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        self.add_rational_prec_round_ref_ref_helper(other, prec, rm)
    }

    /// Adds a [`Float`] and a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] and the [`Rational`] are both are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0.0,0,p)=0.0$
    /// - $f(-0.0,0,p)=-0.0$
    /// - $f(x,-x,p)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_rational_prec_round`] instead. If you know that your target precision is the
    /// precision of the [`Float`] input, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec(Rational::exact_from(1.5), 5);
    /// assert_eq!(sum.to_string(), "4.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec(Rational::exact_from(1.5), 20);
    /// assert_eq!(sum.to_string(), "4.641594");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec(self, other: Rational, prec: u64) -> (Float, Ordering) {
        self.add_rational_prec_round(other, prec, Nearest)
    }

    /// Adds a [`Float`] and a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value and the [`Rational`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0.0,0,p)=0.0$
    /// - $f(-0.0,0,p)=-0.0$
    /// - $f(x,-x,p)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_rational_prec_round_val_ref`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_val_ref(&Rational::exact_from(1.5), 5);
    /// assert_eq!(sum.to_string(), "4.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_val_ref(&Rational::exact_from(1.5), 20);
    /// assert_eq!(sum.to_string(), "4.641594");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Float, Ordering) {
        self.add_rational_prec_round_val_ref(other, prec, Nearest)
    }

    /// Adds a [`Float`] and a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference and the [`Rational`] by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0.0,0,p)=0.0$
    /// - $f(-0.0,0,p)=-0.0$
    /// - $f(x,-x,p)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_rational_prec_round_ref_val`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_ref_val(Rational::exact_from(1.5), 5);
    /// assert_eq!(sum.to_string(), "4.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_ref_val(Rational::exact_from(1.5), 20);
    /// assert_eq!(sum.to_string(), "4.641594");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Float, Ordering) {
        self.add_rational_prec_round_ref_val(other, prec, Nearest)
    }

    /// Adds a [`Float`] and a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] and the [`Rational`] are both are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$
    /// - $f(-\infty,x,p)=-\infty$
    /// - $f(0.0,0,p)=0.0$
    /// - $f(-0.0,0,p)=-0.0$
    /// - $f(x,-x,p)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_rational_prec_round_ref_ref`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_ref_ref(&Rational::exact_from(1.5), 5);
    /// assert_eq!(sum.to_string(), "4.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = Float::from(PI).add_rational_prec_ref_ref(&Rational::exact_from(1.5), 20);
    /// assert_eq!(sum.to_string(), "4.641594");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Float, Ordering) {
        self.add_rational_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Adds a [`Float`] and a [`Rational`], rounding the result with the specified rounding mode.
    /// The [`Float`] and the [`Rational`] are both are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any other [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0,m)=0.0$
    /// - $f(-0.0,0,m)=-0.0$
    /// - $f(0.0,x,m)=f(x,0,m)=f(-0.0,x,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,-x,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,-x,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::add_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the [`Float`] input is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = Float::from(PI).add_rational_round(Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(sum.to_string(), "3.4749259869231262");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_round(Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(sum.to_string(), "3.4749259869231266");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_round(Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(sum.to_string(), "3.4749259869231266");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_round(self, other: Rational, rm: RoundingMode) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.add_rational_prec_round(other, prec, rm)
    }

    /// Adds a [`Float`] and a [`Rational`], rounding the result with the specified rounding mode.
    /// The [`Float`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any other [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0,m)=0.0$
    /// - $f(-0.0,0,m)=-0.0$
    /// - $f(0.0,x,m)=f(x,0,m)=f(-0.0,x,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,-x,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,-x,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::add_rational_prec_round_val_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the [`Float`] input is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(sum.to_string(), "3.4749259869231262");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(sum.to_string(), "3.4749259869231266");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(sum.to_string(), "3.4749259869231266");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_round_val_ref(
        self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.add_rational_prec_round_val_ref(other, prec, rm)
    }

    /// Adds a [`Float`] and a [`Rational`], rounding the result with the specified rounding mode.
    /// The [`Float`] is taken by reference and the [`Float`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any other [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0,m)=0.0$
    /// - $f(-0.0,0,m)=-0.0$
    /// - $f(0.0,x,m)=f(x,0,m)=f(-0.0,x,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,-x,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,-x,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::add_rational_prec_round_ref_val`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the [`Float`] input is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(sum.to_string(), "3.4749259869231262");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(sum.to_string(), "3.4749259869231266");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(sum.to_string(), "3.4749259869231266");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_round_ref_val(
        &self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.add_rational_prec_round_ref_val(other, prec, rm)
    }

    /// Adds a [`Float`] and a [`Rational`], rounding the result with the specified rounding mode.
    /// The [`Float`] and the [`Rational`] are both are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any other [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0,m)=0.0$
    /// - $f(-0.0,0,m)=-0.0$
    /// - $f(0.0,x,m)=f(x,0,m)=f(-0.0,x,m)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,-x,m)=0.0$ if $x$ is nonzero and $m$ is not `Floor`
    /// - $f(x,-x,m)=-0.0$ if $x$ is nonzero and $m$ is `Floor`
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::add_rational_prec_round_ref_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the [`Float`] input is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(sum.to_string(), "3.4749259869231262");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(sum.to_string(), "3.4749259869231266");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     Float::from(PI).add_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(sum.to_string(), "3.4749259869231266");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_rational_round_ref_ref(
        &self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.add_rational_prec_round_ref_ref(other, prec, rm)
    }

    /// Adds a [`Rational`] to a [`Float`] in place, rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any other [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::add_rational_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_rational_prec_assign`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::add_rational_round_assign`] instead. If both of these things are
    /// true, consider using `+=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "3.4");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.5");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.5");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "3.474922");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.474926");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.474926");
    /// ```
    #[inline]
    pub fn add_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        self.add_rational_prec_round_assign_helper(other, prec, rm)
    }

    /// Adds a [`Rational`] to a [`Float`] in place, rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any other [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::add_rational_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::add_rational_prec_assign_ref`] instead. If you know that your target precision is
    /// the precision of the [`Float`] input, consider using
    /// [`Float::add_rational_round_assign_ref`] instead. If both of these things are true, consider
    /// using `+=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "3.4");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.5");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.5");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "3.474922");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.474926");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.474926");
    /// ```
    #[inline]
    pub fn add_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        self.add_rational_prec_round_assign_ref_helper(other, prec, rm)
    }

    /// Adds a [`Rational`] to a [`Float`] in place, rounding the result to the nearest value of the
    /// specified precision. The [`Rational`] is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any other [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::add_rational_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_rational_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the two inputs, consider using `+=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_assign(Rational::exact_from(1.5), 5),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_assign(Rational::exact_from(1.5), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.641594");
    /// ```
    #[inline]
    pub fn add_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        self.add_rational_prec_round_assign(other, prec, Nearest)
    }

    /// Adds a [`Rational`] to a [`Float`] in place, rounding the result to the nearest value of the
    /// specified precision. The [`Rational`] is taken by reference. An [`Ordering`] is returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any other [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::add_rational_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_rational_prec_round_assign_ref`] instead. If you know that your target
    /// precision is the maximum of the precisions of the two inputs, consider using `+=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(other.significant_bits(),
    /// prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_assign_ref(&Rational::exact_from(1.5), 5),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_prec_assign_ref(&Rational::exact_from(1.5), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.641594");
    /// ```
    #[inline]
    pub fn add_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        self.add_rational_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Adds a [`Rational`] to a [`Float`] in place, rounding the result with the specified rounding
    /// mode. The [`Rational`] is taken by value. An [`Ordering`] is returned, indicating whether
    /// the rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are
    /// not comparable to any other [`Float`], whenever this function sets the [`Float`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input [`Float`]. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the input [`Float`].
    ///
    /// See the [`Float::add_rational_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::add_rational_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `+=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the input [`Float`] is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_round_assign(Rational::from_unsigneds(1u8, 3), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "3.4749259869231262");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_round_assign(Rational::from_unsigneds(1u8, 3), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.4749259869231266");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_round_assign(Rational::from_unsigneds(1u8, 3), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.4749259869231266");
    /// ```
    #[inline]
    pub fn add_rational_round_assign(&mut self, other: Rational, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.add_rational_prec_round_assign(other, prec, rm)
    }

    /// Adds a [`Rational`] to a [`Float`] in place, rounding the result with the specified rounding
    /// mode. The [`Rational`] is taken by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded sum is less than, equal to, or greater than the exact sum. Although
    /// `NaN`s are not comparable to any other [`Float`], whenever this function sets the [`Float`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input [`Float`]. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\epsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\epsilon| < 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the input [`Float`].
    ///
    /// See the [`Float::add_rational_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::add_rational_prec_round_assign_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `+=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the input [`Float`] is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_round_assign_ref(&Rational::from_unsigneds(1u8, 3), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "3.4749259869231262");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_round_assign_ref(&Rational::from_unsigneds(1u8, 3), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.4749259869231266");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_rational_round_assign_ref(&Rational::from_unsigneds(1u8, 3), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "3.4749259869231266");
    /// ```
    #[inline]
    pub fn add_rational_round_assign_ref(
        &mut self,
        other: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.add_rational_prec_round_assign_ref(other, prec, rm)
    }
}

impl Add<Float> for Float {
    type Output = Float;

    /// Adds two [`Float`]s, taking both by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,-\infty)=f(-\infty,\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0)=0.0$
    /// - $f(-0.0,-0.0)=-0.0$
    /// - $f(0.0,-0.0)=f(-0.0,0.0)=0.0$
    /// - $f(0.0,x)=f(x,0.0)=f(-0.0,x)=f(x,-0.0)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,-x)=0.0$ if $x$ is finite and nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::add_prec`]
    /// instead. If you want to specify the output precision, consider using [`Float::add_round`].
    /// If you want both of these things, consider using [`Float::add_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((Float::from(1.5) + Float::NAN).is_nan());
    /// assert_eq!(Float::from(1.5) + Float::INFINITY, Float::INFINITY);
    /// assert_eq!(
    ///     Float::from(1.5) + Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert!((Float::INFINITY + Float::NEGATIVE_INFINITY).is_nan());
    ///
    /// assert_eq!(Float::from(1.5) + Float::from(2.5), 4.0);
    /// assert_eq!(Float::from(1.5) + Float::from(-2.5), -1.0);
    /// assert_eq!(Float::from(-1.5) + Float::from(2.5), 1.0);
    /// assert_eq!(Float::from(-1.5) + Float::from(-2.5), -4.0);
    /// ```
    #[inline]
    fn add(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.add_prec_round(other, prec, Nearest).0
    }
}

impl<'a> Add<&'a Float> for Float {
    type Output = Float;

    /// Adds two [`Float`]s, taking the first by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,-\infty)=f(-\infty,\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0)=0.0$
    /// - $f(-0.0,-0.0)=-0.0$
    /// - $f(0.0,-0.0)=f(-0.0,0.0)=0.0$
    /// - $f(0.0,x)=f(x,0.0)=f(-0.0,x)=f(x,-0.0)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,-x)=0.0$ if $x$ is finite and nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_prec_val_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::add_round_val_ref`]. If you want both of these things, consider using
    /// [`Float::add_prec_round_val_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((Float::from(1.5) + &Float::NAN).is_nan());
    /// assert_eq!(Float::from(1.5) + &Float::INFINITY, Float::INFINITY);
    /// assert_eq!(
    ///     Float::from(1.5) + &Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert!((Float::INFINITY + &Float::NEGATIVE_INFINITY).is_nan());
    ///
    /// assert_eq!(Float::from(1.5) + &Float::from(2.5), 4.0);
    /// assert_eq!(Float::from(1.5) + &Float::from(-2.5), -1.0);
    /// assert_eq!(Float::from(-1.5) + &Float::from(2.5), 1.0);
    /// assert_eq!(Float::from(-1.5) + &Float::from(-2.5), -4.0);
    /// ```
    #[inline]
    fn add(self, other: &'a Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.add_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl<'a> Add<Float> for &'a Float {
    type Output = Float;

    /// Adds two [`Float`]s, taking the first by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,-\infty)=f(-\infty,\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0)=0.0$
    /// - $f(-0.0,-0.0)=-0.0$
    /// - $f(0.0,-0.0)=f(-0.0,0.0)=0.0$
    /// - $f(0.0,x)=f(x,0.0)=f(-0.0,x)=f(x,-0.0)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,-x)=0.0$ if $x$ is finite and nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_prec_ref_val`] instead. If you want to specify the output precision, consider
    /// using [`Float::add_round_ref_val`]. If you want both of these things, consider using
    /// [`Float::add_prec_round_ref_val`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::from(1.5) + Float::NAN).is_nan());
    /// assert_eq!(&Float::from(1.5) + Float::INFINITY, Float::INFINITY);
    /// assert_eq!(
    ///     &Float::from(1.5) + Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert!((&Float::INFINITY + Float::NEGATIVE_INFINITY).is_nan());
    ///
    /// assert_eq!(&Float::from(1.5) + Float::from(2.5), 4.0);
    /// assert_eq!(&Float::from(1.5) + Float::from(-2.5), -1.0);
    /// assert_eq!(&Float::from(-1.5) + Float::from(2.5), 1.0);
    /// assert_eq!(&Float::from(-1.5) + Float::from(-2.5), -4.0);
    /// ```
    #[inline]
    fn add(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.add_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl<'a, 'b> Add<&'a Float> for &'b Float {
    type Output = Float;

    /// Adds two [`Float`]s, taking both by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,-\infty)=f(-\infty,\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(0.0,0.0)=0.0$
    /// - $f(-0.0,-0.0)=-0.0$
    /// - $f(0.0,-0.0)=f(-0.0,0.0)=0.0$
    /// - $f(0.0,x)=f(x,0.0)=f(-0.0,x)=f(x,-0.0)=x$ if $x$ is not NaN and $x$ is nonzero
    /// - $f(x,-x)=0.0$ if $x$ is finite and nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_prec_ref_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::add_round_ref_ref`]. If you want both of these things, consider using
    /// [`Float::add_prec_round_ref_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::from(1.5) + &Float::NAN).is_nan());
    /// assert_eq!(&Float::from(1.5) + &Float::INFINITY, Float::INFINITY);
    /// assert_eq!(
    ///     &Float::from(1.5) + &Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert!((&Float::INFINITY + &Float::NEGATIVE_INFINITY).is_nan());
    ///
    /// assert_eq!(&Float::from(1.5) + &Float::from(2.5), 4.0);
    /// assert_eq!(&Float::from(1.5) + &Float::from(-2.5), -1.0);
    /// assert_eq!(&Float::from(-1.5) + &Float::from(2.5), 1.0);
    /// assert_eq!(&Float::from(-1.5) + &Float::from(-2.5), -4.0);
    /// ```
    #[inline]
    fn add(self, other: &'a Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.add_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl AddAssign<Float> for Float {
    /// Adds a [`Float`] to a [`Float`] in place, taking the [`Float`] on the right-hand side by
    /// value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases: See the `+` documenation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::add_round_assign`]. If you want both of these things, consider using
    /// [`Float::add_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1.5);
    /// x += Float::NAN;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x += Float::INFINITY;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from(1.5);
    /// x += Float::NEGATIVE_INFINITY;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x += Float::NEGATIVE_INFINITY;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x += Float::from(2.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(1.5);
    /// x += Float::from(-2.5);
    /// assert_eq!(x, -1.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x += Float::from(2.5);
    /// assert_eq!(x, 1.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x += Float::from(-2.5);
    /// assert_eq!(x, -4.0);
    /// ```
    #[inline]
    fn add_assign(&mut self, other: Float) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.add_prec_round_assign(other, prec, Nearest);
    }
}

impl<'a> AddAssign<&'a Float> for Float {
    /// Adds a [`Float`] to a [`Float`] in place, taking the [`Float`] on the right-hand side by
    /// reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases: See the `+` documenation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_prec_assign_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::add_round_assign_ref`]. If you want both of these things, consider
    /// using [`Float::add_prec_round_assign_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1.5);
    /// x += &Float::NAN;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x += &Float::INFINITY;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from(1.5);
    /// x += &Float::NEGATIVE_INFINITY;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x += &Float::NEGATIVE_INFINITY;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x += &Float::from(2.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(1.5);
    /// x += &Float::from(-2.5);
    /// assert_eq!(x, -1.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x += &Float::from(2.5);
    /// assert_eq!(x, 1.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x += &Float::from(-2.5);
    /// assert_eq!(x, -4.0);
    /// ```
    #[inline]
    fn add_assign(&mut self, other: &Float) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.add_prec_round_assign_ref(other, prec, Nearest);
    }
}

impl Add<Rational> for Float {
    type Output = Float;

    /// Adds a [`Float`] and a [`Rational`], taking both by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the sum is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=\text{NaN}$
    /// - $f(\infty,x)=\infty$
    /// - $f(-\infty,x)=-\infty$
    /// - $f(0.0,0)=0.0$
    /// - $f(-0.0,0)=-0.0$
    /// - $f(0.0,x)=f(x,0)=f(-0.0,x)=x$
    /// - $f(x,-x)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_rational_prec`] instead. If you want to specify the output precision, consider
    /// using [`Float::add_rational_round`]. If you want both of these things, consider using
    /// [`Float::add_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Float::NAN + Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(Float::INFINITY + Rational::exact_from(1.5), Float::INFINITY);
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY + Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    ///
    /// assert_eq!(Float::from(2.5) + Rational::exact_from(1.5), 4.0);
    /// assert_eq!(Float::from(2.5) + Rational::exact_from(-1.5), 1.0);
    /// assert_eq!(Float::from(-2.5) + Rational::exact_from(1.5), -1.0);
    /// assert_eq!(Float::from(-2.5) + Rational::exact_from(-1.5), -4.0);
    /// ```
    #[inline]
    fn add(self, other: Rational) -> Float {
        let prec = self.significant_bits();
        self.add_rational_prec_round(other, prec, Nearest).0
    }
}

impl<'a> Add<&'a Rational> for Float {
    type Output = Float;

    /// Adds a [`Float`] and a [`Rational`], taking the [`Float`] by value and the [`Rational`] by
    /// reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the sum is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=\text{NaN}$
    /// - $f(\infty,x)=\infty$
    /// - $f(-\infty,x)=-\infty$
    /// - $f(0.0,0)=0.0$
    /// - $f(-0.0,0)=-0.0$
    /// - $f(0.0,x)=f(x,0)=f(-0.0,x)=x$
    /// - $f(x,-x)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_rational_prec_val_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::add_rational_round_val_ref`]. If you want both of these things,
    /// consider using [`Float::add_rational_prec_round_val_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Float::NAN + &Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(
    ///     Float::INFINITY + &Rational::exact_from(1.5),
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY + &Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    ///
    /// assert_eq!(Float::from(2.5) + &Rational::exact_from(1.5), 4.0);
    /// assert_eq!(Float::from(2.5) + &Rational::exact_from(-1.5), 1.0);
    /// assert_eq!(Float::from(-2.5) + &Rational::exact_from(1.5), -1.0);
    /// assert_eq!(Float::from(-2.5) + &Rational::exact_from(-1.5), -4.0);
    /// ```
    #[inline]
    fn add(self, other: &Rational) -> Float {
        let prec = self.significant_bits();
        self.add_rational_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl<'a> Add<Rational> for &'a Float {
    type Output = Float;

    /// Adds a [`Float`] and a [`Rational`], taking the [`Float`] by reference and the [`Rational`]
    /// by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the sum is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=\text{NaN}$
    /// - $f(\infty,x)=\infty$
    /// - $f(-\infty,x)=-\infty$
    /// - $f(0.0,0)=0.0$
    /// - $f(-0.0,0)=-0.0$
    /// - $f(0.0,x)=f(x,0)=f(-0.0,x)=x$
    /// - $f(x,-x)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_rational_prec_ref_val`] instead. If you want to specify the output precision,
    /// consider using [`Float::add_rational_round_ref_val`]. If you want both of these things,
    /// consider using [`Float::add_rational_prec_round_ref_val`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Float::NAN + Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(
    ///     &Float::INFINITY + Rational::exact_from(1.5),
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::NEGATIVE_INFINITY + Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    ///
    /// assert_eq!(&Float::from(2.5) + Rational::exact_from(1.5), 4.0);
    /// assert_eq!(&Float::from(2.5) + Rational::exact_from(-1.5), 1.0);
    /// assert_eq!(&Float::from(-2.5) + Rational::exact_from(1.5), -1.0);
    /// assert_eq!(&Float::from(-2.5) + Rational::exact_from(-1.5), -4.0);
    /// ```
    #[inline]
    fn add(self, other: Rational) -> Float {
        let prec = self.significant_bits();
        self.add_rational_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl<'a, 'b> Add<&'a Rational> for &'b Float {
    type Output = Float;

    /// Adds a [`Float`] and a [`Rational`], taking both by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the sum is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=\text{NaN}$
    /// - $f(\infty,x)=\infty$
    /// - $f(-\infty,x)=-\infty$
    /// - $f(0.0,0)=0.0$
    /// - $f(-0.0,0)=-0.0$
    /// - $f(0.0,x)=f(x,0)=f(-0.0,x)=x$
    /// - $f(x,-x)=0.0$ if $x$ is nonzero
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_rational_prec_ref_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::add_rational_round_ref_ref`]. If you want both of these things,
    /// consider using [`Float::add_rational_prec_round_ref_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Float::NAN + &Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(
    ///     &Float::INFINITY + &Rational::exact_from(1.5),
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::NEGATIVE_INFINITY + &Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    ///
    /// assert_eq!(&Float::from(2.5) + &Rational::exact_from(1.5), 4.0);
    /// assert_eq!(&Float::from(2.5) + &Rational::exact_from(-1.5), 1.0);
    /// assert_eq!(&Float::from(-2.5) + &Rational::exact_from(1.5), -1.0);
    /// assert_eq!(&Float::from(-2.5) + &Rational::exact_from(-1.5), -4.0);
    /// ```
    #[inline]
    fn add(self, other: &Rational) -> Float {
        let prec = self.significant_bits();
        self.add_rational_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl AddAssign<Rational> for Float {
    /// Adds a [`Rational`] to a [`Float`] in place, taking the [`Rational`] by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the sum is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// See the `+` documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_rational_prec_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::add_rational_round_assign`]. If you want both of these things,
    /// consider using [`Float::add_rational_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::NAN;
    /// x += Rational::exact_from(1.5);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x += Rational::exact_from(1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x += Rational::exact_from(1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(2.5);
    /// x += Rational::exact_from(1.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(2.5);
    /// x += Rational::exact_from(-1.5);
    /// assert_eq!(x, 1.0);
    ///
    /// let mut x = Float::from(-2.5);
    /// x += Rational::exact_from(1.5);
    /// assert_eq!(x, -1.0);
    ///
    /// let mut x = Float::from(-2.5);
    /// x += Rational::exact_from(-1.5);
    /// assert_eq!(x, -4.0);
    /// ```
    #[inline]
    fn add_assign(&mut self, other: Rational) {
        let prec = self.significant_bits();
        self.add_rational_prec_round_assign(other, prec, Nearest);
    }
}

impl<'a> AddAssign<&'a Rational> for Float {
    /// Adds a [`Rational`] to a [`Float`] in place, taking the [`Rational`] by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the sum is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// See the `+` documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_rational_prec_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::add_rational_round_assign`]. If you want both of these things,
    /// consider using [`Float::add_rational_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::NAN;
    /// x += &Rational::exact_from(1.5);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x += &Rational::exact_from(1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x += &Rational::exact_from(1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(2.5);
    /// x += &Rational::exact_from(1.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(2.5);
    /// x += &Rational::exact_from(-1.5);
    /// assert_eq!(x, 1.0);
    ///
    /// let mut x = Float::from(-2.5);
    /// x += &Rational::exact_from(1.5);
    /// assert_eq!(x, -1.0);
    ///
    /// let mut x = Float::from(-2.5);
    /// x += &Rational::exact_from(-1.5);
    /// assert_eq!(x, -4.0);
    /// ```
    #[inline]
    fn add_assign(&mut self, other: &Rational) {
        let prec = self.significant_bits();
        self.add_rational_prec_round_assign_ref(other, prec, Nearest);
    }
}

impl Add<Float> for Rational {
    type Output = Float;

    /// Adds a [`Rational`] and a [`Float`], taking both by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the sum is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,\infty)=\infty$
    /// - $f(x,-\infty)=-\infty$
    /// - $f(0,0.0)=0.0$
    /// - $f(0,-0.0)=-0.0$
    /// - $f(x,0.0)=f(x,0)=f(-0.0,x)=x$
    /// - $f(x,-x)=0.0$ if $x$ is nonzero
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Rational::exact_from(1.5) + Float::NAN).is_nan());
    /// assert_eq!(Rational::exact_from(1.5) + Float::INFINITY, Float::INFINITY);
    /// assert_eq!(
    ///     Rational::exact_from(1.5) + Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    ///
    /// assert_eq!(Rational::exact_from(1.5) + Float::from(2.5), 4.0);
    /// assert_eq!(Rational::exact_from(1.5) + Float::from(-2.5), -1.0);
    /// assert_eq!(Rational::exact_from(-1.5) + Float::from(2.5), 1.0);
    /// assert_eq!(Rational::exact_from(-1.5) + Float::from(-2.5), -4.0);
    /// ```
    #[inline]
    fn add(self, other: Float) -> Float {
        let prec = other.significant_bits();
        other.add_rational_prec_round(self, prec, Nearest).0
    }
}

impl<'a> Add<&'a Float> for Rational {
    type Output = Float;

    /// Adds a [`Rational`] and a [`Float`], taking the [`Rational`] by value and the [`Float`] by
    /// reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the sum is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,\infty)=\infty$
    /// - $f(x,-\infty)=-\infty$
    /// - $f(0,0.0)=0.0$
    /// - $f(0,-0.0)=-0.0$
    /// - $f(x,0.0)=f(x,0)=f(-0.0,x)=x$
    /// - $f(x,-x)=0.0$ if $x$ is nonzero
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Rational::exact_from(1.5) + &Float::NAN).is_nan());
    /// assert_eq!(
    ///     Rational::exact_from(1.5) + &Float::INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     Rational::exact_from(1.5) + &Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    ///
    /// assert_eq!(Rational::exact_from(1.5) + &Float::from(2.5), 4.0);
    /// assert_eq!(Rational::exact_from(1.5) + &Float::from(-2.5), -1.0);
    /// assert_eq!(Rational::exact_from(-1.5) + &Float::from(2.5), 1.0);
    /// assert_eq!(Rational::exact_from(-1.5) + &Float::from(-2.5), -4.0);
    /// ```
    #[inline]
    fn add(self, other: &Float) -> Float {
        let prec = other.significant_bits();
        other.add_rational_prec_round_ref_val(self, prec, Nearest).0
    }
}

impl<'a> Add<Float> for &'a Rational {
    type Output = Float;

    /// Adds a [`Rational`] and a [`Float`], taking the [`Rational`] by reference and the [`Float`]
    /// by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the sum is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,\infty)=\infty$
    /// - $f(x,-\infty)=-\infty$
    /// - $f(0,0.0)=0.0$
    /// - $f(0,-0.0)=-0.0$
    /// - $f(x,0.0)=f(x,0)=f(-0.0,x)=x$
    /// - $f(x,-x)=0.0$ if $x$ is nonzero
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Rational::exact_from(1.5) + Float::NAN).is_nan());
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) + Float::INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) + Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    ///
    /// assert_eq!(&Rational::exact_from(1.5) + Float::from(2.5), 4.0);
    /// assert_eq!(&Rational::exact_from(1.5) + Float::from(-2.5), -1.0);
    /// assert_eq!(&Rational::exact_from(-1.5) + Float::from(2.5), 1.0);
    /// assert_eq!(&Rational::exact_from(-1.5) + Float::from(-2.5), -4.0);
    /// ```
    #[inline]
    fn add(self, other: Float) -> Float {
        let prec = other.significant_bits();
        other.add_rational_prec_round_val_ref(self, prec, Nearest).0
    }
}

impl<'a, 'b> Add<&'a Float> for &'b Rational {
    type Output = Float;

    /// Adds a [`Rational`] and a [`Float`], taking both by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the sum is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x+y+\epsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\epsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\epsilon| < 2^{\lfloor\log_2 |x+y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=\text{NaN}$
    /// - $f(x,\infty)=\infty$
    /// - $f(x,-\infty)=-\infty$
    /// - $f(0,0.0)=0.0$
    /// - $f(0,-0.0)=-0.0$
    /// - $f(x,0.0)=f(x,0)=f(-0.0,x)=x$
    /// - $f(x,-x)=0.0$ if $x$ is nonzero
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Rational::exact_from(1.5) + &Float::NAN).is_nan());
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) + &Float::INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) + &Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    ///
    /// assert_eq!(&Rational::exact_from(1.5) + &Float::from(2.5), 4.0);
    /// assert_eq!(&Rational::exact_from(1.5) + &Float::from(-2.5), -1.0);
    /// assert_eq!(&Rational::exact_from(-1.5) + &Float::from(2.5), 1.0);
    /// assert_eq!(&Rational::exact_from(-1.5) + &Float::from(-2.5), -4.0);
    /// ```
    #[inline]
    fn add(self, other: &Float) -> Float {
        let prec = other.significant_bits();
        other.add_rational_prec_round_ref_ref(self, prec, Nearest).0
    }
}
