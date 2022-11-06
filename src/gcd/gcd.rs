use num::Zero;
use std::ops::Rem;

pub fn gcd_calc<T>(x: T, y: T) -> T
where
    T: Zero,
    for<'a> &'a T: Rem<Output = T>,
{
    if x.is_zero() {
        y
    } else if y.is_zero() {
        x
    } else {
        let r = &x % &y;
        gcd_calc(y, r)
    }
}

#[test]
fn gcd_test() {
    use num::{BigInt, Num};

    assert_eq!(
        gcd_calc(BigInt::from(10), BigInt::from(30)),
        BigInt::from(10)
    );
    assert_eq!(
        gcd_calc(
            BigInt::from_str_radix("20689856962082115885041972726487268355243056299479", 10)
                .unwrap(),
            BigInt::from_str_radix("4014476939333036189094441199026045136645885247730", 10)
                .unwrap()
        ),
        BigInt::from(127)
    );
    assert_eq!(gcd_calc(10, 30), 10);
}
