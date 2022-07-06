#[derive(Debug)]
pub enum Car {
    Bmw,
    Mercedes(String),
    Kia(i32),
    Toyota(bool),
}

pub struct Person {
    pub person_name: String,
    pub last_name: String,
    pub age: u32,
    pub card_number: String,
    pub car: Car,
}

impl Person {
    pub fn print_person_info(&self) {
        println!("Name is {} {}, age is {}, card number is {}, kind of car is {:#?}",
                 self.person_name, self.last_name, self.age, self.card_number, self.car
        );
        match self.car {
            Car::Bmw =>  println!("BMW"),
            Car::Toyota(bool) => {
                match bool {
                    true => println!("true"),
                    false => println!("false"),
                }
            },
            // Car::Mercedes(_) | Car::Kia(_) => println!("Car"),
            _ => println!("Car"),
        }
    }

    pub fn check_enums() {
        let mercedes = Car::Mercedes("mercedes".to_string());
        let toyota = Car::Toyota(true);
        let bmw = Car::Bmw;
        println!("{:#?}, {:#?}, {:#?}", bmw, mercedes, toyota)
    }
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub struct CoinItem {
    pub coin: Coin
}

impl CoinItem {
    pub fn get_value_in_cent(&self) -> u8 {
        match self.coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

pub fn handle_option(item: Option<bool>) {
    match item {
        Some(bool)  => println!("item is {}", bool),
        None => println!("There is no item"),
    }

    if let Some(bool) = item {
        println!("if let {}", bool);
    }
}



