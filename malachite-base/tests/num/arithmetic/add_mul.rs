use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_add_mul() {
    fn test<T>(x: T, y: T, z: T, out: T)
    where
        T: PrimitiveInteger,
    {
        assert_eq!(x.add_mul(y, z), out);

        let mut x = x;
        x.add_mul_assign(y, z);
        assert_eq!(x, out);
    };
    test::<u8>(2, 3, 7, 23);
    test::<u32>(7, 5, 10, 57);
    test::<u64>(123, 456, 789, 359_907);
    test::<i32>(123, -456, 789, -359_661);
    test::<i128>(-123, 456, 789, 359_661);
}