mod fizzbuzz;
mod gcd;

use fizzbuzz::fizzbuzz::{fizz_buzz_calc, fizz_buzz_calc_int};
use gcd::gcd::gcd_calc;
use num::{BigInt, Num};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn gcd(x: &str, y: &str) -> String {
    gcd_calc(
        BigInt::from_str_radix(&x, 10).expect("error parsing argument"),
        BigInt::from_str_radix(&y, 10).expect("error parsing argument"),
    )
    .to_string()
}

#[wasm_bindgen]
pub fn fizzbuzz(number: &str) -> String {
    let number = BigInt::from_str_radix(number, 10).expect("error parsing argument");
    fizz_buzz_calc(number)
}

#[wasm_bindgen]
pub fn fizzbuzz_int(number: u32) -> String {
    fizz_buzz_calc_int(number)
}

#[wasm_bindgen]
pub fn fizzbuzz_bigint(number: u64) -> String {
    fizz_buzz_calc_int(number)
}

#[test]
fn it_works() {
    let result = greet("World");
    assert_eq!(result, "Hello, World!");
}

#[test]
fn gcd_test() {
    assert_eq!(
        gcd_calc(BigInt::from(10), BigInt::from(30)),
        BigInt::from(10)
    );
    assert_eq!(
        gcd_calc(BigInt::from(6171373), BigInt::from(1513733)),
        BigInt::from(116441)
    );
    assert_eq!(gcd("1763", "1927"), "41");
    assert_eq!(gcd("6171373", "1513733"), "116441");
    assert_eq!(gcd("10", "30"), "10");
    assert_eq!(gcd("1763", "1927"), "41");
    assert_eq!(gcd("6171373", "1513733"), "116441");
}

#[test]
fn fizz_buzz_calc_test() {
    assert_eq!(fizz_buzz_calc(1), "1");
    assert_eq!(fizz_buzz_calc(3), "Fizz");
    assert_eq!(fizz_buzz_calc(5), "Buzz");
    assert_eq!(fizz_buzz_calc(15), "FizzBuzz");
    assert_eq!(
        fizz_buzz_calc(BigInt::from_str_radix("18446744073709551616", 10).unwrap()),
        "18446744073709551616"
    );
    assert_eq!(
        fizz_buzz_calc(BigInt::from_str_radix("18446744073709551618", 10).unwrap()),
        "Fizz"
    );
    assert_eq!(
        fizz_buzz_calc(BigInt::from_str_radix("18446744073709551620", 10).unwrap()),
        "Buzz"
    );
    assert_eq!(
        fizz_buzz_calc(BigInt::from_str_radix("18446744073709551630", 10).unwrap()),
        "FizzBuzz"
    );
}

#[test]
fn fizzbuzz_int_test() {
    assert_eq!(fizzbuzz_int(1), "1");
    assert_eq!(fizzbuzz_int(3), "Fizz");
    assert_eq!(fizzbuzz_int(5), "Buzz");
    assert_eq!(fizzbuzz_int(15), "FizzBuzz");
}

#[test]
fn fizzbuzz_test() {
    assert_eq!(fizzbuzz("1"), "1");
    assert_eq!(fizzbuzz("3"), "Fizz");
    assert_eq!(fizzbuzz("5"), "Buzz");
    assert_eq!(fizzbuzz("15"), "FizzBuzz");
    assert_eq!(fizzbuzz("18446744073709551616"), "18446744073709551616");
    assert_eq!(fizzbuzz("18446744073709551618"), "Fizz");
    assert_eq!(fizzbuzz("18446744073709551620"), "Buzz");
    assert_eq!(fizzbuzz("18446744073709551630"), "FizzBuzz");
}

#[test]
#[should_panic(expected = "error parsing argument")]
fn fizzbuzz_panic_test() {
    fizzbuzz("a");
}
