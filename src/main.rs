extern crate core;

use crate::convert_fahrenheit_to_celsius::convert_fahrenheit_to_celsius;
use crate::fibonacci_number::get_fibonacci_number;
use crate::guess_task::guess_task;

mod guess_task;
mod convert_fahrenheit_to_celsius;
mod fibonacci_number;

fn main() {
    guess_task(false);
    convert_fahrenheit_to_celsius(197.0,false);
    get_fibonacci_number(4, true);
}
