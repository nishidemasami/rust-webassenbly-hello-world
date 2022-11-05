use num::Zero;
use std::ops::Rem;

pub fn gcd_calc<T>(x: T, y: T) -> T
where
    T: Zero,
    for<'x> &'x T: Rem<Output = T>,
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
