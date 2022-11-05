use num::Zero;
use std::ops::Rem;

pub fn fizz_buzz_calc<T: Zero + From<u8> + Clone + Rem<Output = T> + ToString>(
    number: &T,
) -> String {
    match (
        (number.clone() % T::from(3)).is_zero(),
        (number.clone() % T::from(5)).is_zero(),
    ) {
        (true, true) => "FizzBuzz".to_string(),
        (true, _) => "Fizz".to_string(),
        (_, true) => "Buzz".to_string(),
        _ => number.to_string(),
    }
}

#[test]
fn fizz_buzz_calc_test() {
    use num::{BigInt, Num};

    assert_eq!(fizz_buzz_calc(&1), "1");
    assert_eq!(fizz_buzz_calc(&3), "Fizz");
    assert_eq!(fizz_buzz_calc(&5), "Buzz");
    assert_eq!(fizz_buzz_calc(&15), "FizzBuzz");
    assert_eq!(
        fizz_buzz_calc(&BigInt::from_str_radix("1", 10).unwrap()),
        "1"
    );
    assert_eq!(
        fizz_buzz_calc(&BigInt::from_str_radix("3", 10).unwrap()),
        "Fizz"
    );
    assert_eq!(
        fizz_buzz_calc(&BigInt::from_str_radix("5", 10).unwrap()),
        "Buzz"
    );
    assert_eq!(
        fizz_buzz_calc(&BigInt::from_str_radix("15", 10).unwrap()),
        "FizzBuzz"
    );
    assert_eq!(
        fizz_buzz_calc(&BigInt::from_str_radix("18446744073709551616", 10).unwrap()),
        "18446744073709551616"
    );
    assert_eq!(
        fizz_buzz_calc(&BigInt::from_str_radix("18446744073709551618", 10).unwrap()),
        "Fizz"
    );
    assert_eq!(
        fizz_buzz_calc(&BigInt::from_str_radix("18446744073709551620", 10).unwrap()),
        "Buzz"
    );
    assert_eq!(
        fizz_buzz_calc(&BigInt::from_str_radix("18446744073709551630", 10).unwrap()),
        "FizzBuzz"
    );
}
