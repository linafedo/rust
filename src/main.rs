extern crate core;

use std::ptr::null;
use crate::convert_fahrenheit_to_celsius::convert_fahrenheit_to_celsius;
use crate::enums::{Car, Coin, CoinItem, handle_option, Person};
use crate::fibonacci_number::get_fibonacci_number;
use crate::guess_task::guess_task;
use crate::structures::{print_area_for_size, Size};

mod guess_task;
mod convert_fahrenheit_to_celsius;
mod fibonacci_number;
mod structures;
mod enums;

fn main() {
    // 1
    guess_task(false);
    // 2
    convert_fahrenheit_to_celsius(197.0,false);
    // 3
    get_fibonacci_number(4, false);
    // 4
    let scale = 2;
    let size = Size{
        height: dbg!(30 * scale),
        width: 8
    };
    print_area_for_size(&size, false);
    println!("{}", size.height);
    println!("{:#?}", &size);
    // 5
    let person =  Person{
        person_name: "Galina".to_string(),
        last_name: "Fedorova".to_string(),
        age: 27,
        card_number: "1111 1111 1111 1111".to_string(),
        car: Car::Kia(32)
    };
    person.print_person_info();
    Person::check_enums();

    let coin = CoinItem{coin: Coin::Quarter};
    println!("Coin is {} cents", coin.get_value_in_cent());

    handle_option(Option::Some(true));
    handle_option(Option::None);


}
