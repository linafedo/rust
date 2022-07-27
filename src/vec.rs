use std::fmt::format;
use std::collections::HashMap;

pub fn vec() {
    let mut v = vec![1, 2, 3];
    v.push(100);
    println!("{}", v.last().unwrap_or(&0));

    let third = v[2];

    println!("{third}");
    for i in &v {
        println!("{}", i);
    }

    let option = v.get(2);
    match option {
        None => println!("Value  is nil"),
        Some(option) => println!("value is {option}")
    }
}

pub enum TypeForVector {
    Int(i32),
    String(String),
    Float(f64),
}

pub struct Vector {
    pub vec: Vec<TypeForVector>,
}

impl Vector {
    fn create_vec() {
        let mut vector = Vector {
            vec: vec![
                TypeForVector::Float(77.9),
                TypeForVector::String("ttt".to_string()),
            ]
        };
    }

    pub fn get_str() {
        let string = "Здравствуйте";
        // let str = &string[0..1]; /// error
        let str = &string[0..2];
        println!("{str}");

        for char in &mut string.chars() {
            println!("{char}")
        }
        for char in &mut string.bytes() {
            println!("{char}")
        }
    }
}

pub fn map() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

pub fn get_mid_value(vec: Vec<i32>) -> i32 {
    let count = vec.len();
    let mut summ = 0;
    for value in vec {
        summ += value;
    }
    summ / i32::try_from(count).expect("Sorry, we have a problem")
}

pub fn get_mediana(vec: &mut Vec<i32>) -> Option<i32> {
    let mut mediana: Option<i32> = None;
    if vec.len() == 0 { return mediana }
    vec.sort();
    let remainder = vec.len() % 2;
    if remainder == 0 {
        let first = vec.len() / 2 - 1;
        let second = vec.len()/2;
        let items = (vec.get(first), vec.get(second));
        if let (Some(first), Some(second)) = items {
            mediana = Some((first + second) / 2);
        }
    }  else {
        let index = vec.len()/2;
        if let Some(item) = vec.get(index) {
            mediana = Some(*item);
        }
    }
    mediana
}



