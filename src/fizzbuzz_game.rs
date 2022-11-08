mod fizzbuzz;

use fizzbuzz::fizzbuzz::fizz_buzz_calc;

fn main() {
    (1u32..=100u32)
        .map(fizz_buzz_calc)
        .for_each(|x| println!("{}", x.to_string()))
}
