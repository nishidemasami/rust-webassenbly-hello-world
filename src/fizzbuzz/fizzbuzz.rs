use num::Zero;
use std::ops::Rem;

pub fn fizz_buzz_calc<T>(number: &T) -> String
where
    T: Zero,
    T: From<u8>,
    T: Clone,
    T: Rem<Output = T>,
    T: ToString,
{
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
