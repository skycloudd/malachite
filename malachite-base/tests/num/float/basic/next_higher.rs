use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use std::panic::catch_unwind;

#[allow(clippy::approx_constant)]
#[test]
pub fn test_next_higher() {
    fn test<T: PrimitiveFloat>(x: T, out: T) {
        assert_eq!(NiceFloat(x.next_higher()), out);
    };

    test::<f32>(f32::NEGATIVE_INFINITY, -f32::MAX_FINITE);
    test::<f32>(-f32::MAX_FINITE, -3.4028233e38);
    test::<f32>(-458.42188, -458.42184);
    test::<f32>(-10.0, -9.999999);
    test::<f32>(-core::f32::consts::PI, -3.1415925);
    test::<f32>(-1.0, -0.99999994);
    test::<f32>(-0.1, -0.099999994);
    test::<f32>(-f32::MIN_POSITIVE_NORMAL, -f32::MAX_SUBNORMAL);
    test::<f32>(-f32::MAX_SUBNORMAL, -1.1754941e-38);
    test::<f32>(-f32::MIN_POSITIVE_SUBNORMAL, -0.0);
    test::<f32>(-0.0, 0.0);
    test::<f32>(0.0, f32::MIN_POSITIVE_SUBNORMAL);
    test::<f32>(f32::MIN_POSITIVE_SUBNORMAL, 3.0e-45);
    test::<f32>(f32::MAX_SUBNORMAL, f32::MIN_POSITIVE_NORMAL);
    test::<f32>(f32::MIN_POSITIVE_NORMAL, 1.1754945e-38);
    test::<f32>(0.1, 0.10000001);
    test::<f32>(0.99999994, 1.0);
    test::<f32>(1.0, 1.0000001);
    test::<f32>(1.0000001, 1.0000002);
    test::<f32>(3.1415925, core::f32::consts::PI);
    test::<f32>(core::f32::consts::PI, 3.141593);
    test::<f32>(3.141593, 3.1415932);
    test::<f32>(10.0, 10.000001);
    test::<f32>(f32::MAX_FINITE, f32::POSITIVE_INFINITY);

    test::<f64>(f64::NEGATIVE_INFINITY, -f64::MAX_FINITE);
    test::<f64>(-f64::MAX_FINITE, -1.7976931348623155e308);
    test::<f64>(-10.0, -9.999999999999998);
    test::<f64>(-core::f64::consts::PI, -3.1415926535897927);
    test::<f64>(-1.0, -0.9999999999999999);
    test::<f64>(-0.1, -0.09999999999999999);
    test::<f64>(-f64::MIN_POSITIVE_NORMAL, -f64::MAX_SUBNORMAL);
    test::<f64>(-f64::MAX_SUBNORMAL, -2.2250738585072004e-308);
    test::<f64>(-f64::MIN_POSITIVE_SUBNORMAL, -0.0);
    test::<f64>(-0.0, 0.0);
    test::<f64>(0.0, f64::MIN_POSITIVE_SUBNORMAL);
    test::<f64>(f64::MIN_POSITIVE_SUBNORMAL, 1.0e-323);
    test::<f64>(f64::MAX_SUBNORMAL, f64::MIN_POSITIVE_NORMAL);
    test::<f64>(f64::MIN_POSITIVE_NORMAL, 2.225073858507202e-308);
    test::<f64>(1.9261352099337372e-256, 1.9261352099337375e-256);
    test::<f64>(0.1, 0.10000000000000002);
    test::<f64>(0.9999999999999999, 1.0);
    test::<f64>(1.0, 1.0000000000000002);
    test::<f64>(1.0000000000000002, 1.0000000000000004);
    test::<f64>(3.1415926535897927, core::f64::consts::PI);
    test::<f64>(core::f64::consts::PI, 3.1415926535897936);
    test::<f64>(3.1415926535897936, 3.141592653589794);
    test::<f64>(10.0, 10.000000000000002);
    test::<f64>(f64::MAX_FINITE, f64::POSITIVE_INFINITY);
}

fn next_higher_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(T::NAN.next_higher());
    assert_panic!(T::POSITIVE_INFINITY.next_higher());
}

#[test]
pub fn next_higher_fail() {
    apply_fn_to_primitive_floats!(next_higher_fail_helper);
}
