use num::Zero;
use std::ops::Rem;

pub fn fizz_buzz_calc<'a, T, U>(number: T) -> Box<dyn 'a + ToString>
where
    T: 'a,
    T: From<u8>,
    T: ToString,
    for<'b> &'b T: Rem<Output = U>,
    U: Zero,
{
    match (
        (&number % &T::from(3)).is_zero(),
        (&number % &T::from(5)).is_zero(),
    ) {
        (true, true) => Box::new("FizzBuzz"),
        (true, _) => Box::new("Fizz"),
        (_, true) => Box::new("Buzz"),
        _ => Box::new(number),
    }
}

pub fn fizz_buzz_calc_int<'a, T: 'a + Copy + From<u8> + ToString + Rem<T, Output = U>, U: Zero>(
    number: T,
) -> Box<dyn 'a + ToString> {
    match (
        (number % T::from(3)).is_zero(),
        (number % T::from(5)).is_zero(),
    ) {
        (true, true) => Box::new("FizzBuzz"),
        (true, _) => Box::new("Fizz"),
        (_, true) => Box::new("Buzz"),
        _ => Box::new(number),
    }
}

#[test]
fn fizz_buzz_calc_int_test() {
    assert_eq!(fizz_buzz_calc_int(1).to_string(), "1");
    assert_eq!(fizz_buzz_calc_int(3).to_string(), "Fizz");
    assert_eq!(fizz_buzz_calc_int(5).to_string(), "Buzz");
    assert_eq!(fizz_buzz_calc_int(15).to_string(), "FizzBuzz");
}

#[test]
fn fizz_buzz_calc_test() {
    use num::{BigInt, Num};

    assert_eq!(fizz_buzz_calc(1).to_string(), "1");
    assert_eq!(fizz_buzz_calc(3).to_string(), "Fizz");
    assert_eq!(fizz_buzz_calc(5).to_string(), "Buzz");
    assert_eq!(fizz_buzz_calc(15).to_string(), "FizzBuzz");
    assert_eq!(
        fizz_buzz_calc(18446744073709551601u64).to_string(),
        "18446744073709551601"
    );
    assert_eq!(fizz_buzz_calc(18446744073709551603u64).to_string(), "Fizz");
    assert_eq!(fizz_buzz_calc(18446744073709551605u64).to_string(), "Buzz");
    assert_eq!(
        fizz_buzz_calc(18446744073709551615u64).to_string(),
        "FizzBuzz"
    );
    assert_eq!(
        fizz_buzz_calc(BigInt::from_str_radix("1", 10).unwrap()).to_string(),
        "1"
    );
    assert_eq!(
        fizz_buzz_calc(BigInt::from_str_radix("3", 10).unwrap()).to_string(),
        "Fizz"
    );
    assert_eq!(
        fizz_buzz_calc(BigInt::from_str_radix("5", 10).unwrap()).to_string(),
        "Buzz"
    );
    assert_eq!(
        fizz_buzz_calc(BigInt::from_str_radix("15", 10).unwrap()).to_string(),
        "FizzBuzz"
    );
    assert_eq!(
        fizz_buzz_calc(BigInt::from_str_radix("18446744073709551616", 10).unwrap()).to_string(),
        "18446744073709551616"
    );
    assert_eq!(
        fizz_buzz_calc(BigInt::from_str_radix("18446744073709551618", 10).unwrap()).to_string(),
        "Fizz"
    );
    assert_eq!(
        fizz_buzz_calc(BigInt::from_str_radix("18446744073709551620", 10).unwrap()).to_string(),
        "Buzz"
    );
    assert_eq!(
        fizz_buzz_calc(BigInt::from_str_radix("18446744073709551630", 10).unwrap()).to_string(),
        "FizzBuzz"
    );
}
