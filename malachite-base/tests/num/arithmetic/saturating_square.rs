use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_saturating_square() {
    fn test<T: PrimitiveInteger>(x: T, out: T) {
        assert_eq!(x.saturating_square(), out);

        let mut x = x;
        x.saturating_square_assign();
        assert_eq!(x, out);
    };
    test::<u8>(0, 0);
    test::<i16>(1, 1);
    test::<u32>(2, 4);
    test::<i64>(3, 9);
    test::<u128>(10, 100);
    test::<isize>(123, 15_129);
    test::<u32>(1_000, 1_000_000);

    test::<i16>(-1, 1);
    test::<i32>(-2, 4);
    test::<i64>(-3, 9);
    test::<i128>(-10, 100);
    test::<isize>(-123, 15_129);
    test::<i32>(-1_000, 1_000_000);

    test::<u16>(1_000, u16::MAX);
    test::<i16>(-1_000, i16::MAX);
}