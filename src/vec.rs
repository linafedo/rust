use std::borrow::BorrowMut;
use std::cmp::max;
use std::fmt::format;
use std::collections::HashMap;
use std::str::Chars;
use std::io;

pub fn vec() {
    let mut v = vec![1, 2, 3];
    v.push(100);
    println!("{}", v.last().unwrap_or(&0));

    for i in &v {
        println!("{}", i);
    }

    let option = v.get(2);
    match option {
        None => println!("Value  is nil"),
        Some(_option) => println!("value is {_option}")
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
        let mut _vector = Vector {
            vec: vec![
                TypeForVector::Float(77.9),
                TypeForVector::String("ttt".to_string()),
            ]
        };
    }

    pub fn get_str() {
        let string = "Здравствуйте";
        // let str = &string[0..1]; /// error
        let _str = &string[0..2];
        println!("{_str}");

        for _char in &mut string.chars() {
            println!("{_char}")
        }
        for _char in &mut string.bytes() {
            println!("{_char}")
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

pub  fn get_mode_of_list(vec: Vec<i32>) -> Vec<i32> {
    let mut modes: Vec<i32> = Vec::new();
    let mut max_count = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();
    for item in vec {
        let mut count = map.entry(item).or_insert(0);
        *count += 1;
    }
    for  (key, value) in map.iter(){
        if modes.is_empty() {
            max_count = *value;
            modes.push(*key);
        } else if max_count == *value {
            modes.push(*key);
        } else if  max_count < *value{
            modes.clear();
            max_count = *value;
            modes.push(*key);
        }
    }
    modes
}

pub fn pig_latin_transform() {
    println!("Enter words");

    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    if str.is_empty() || !str.is_ascii() {
        println!("String must be not empty");
        return
    };

    let mut new_str = String::new();
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'y'];

    for word in str.split_whitespace() {
        let first_char = &word.chars().nth(0).unwrap();

        if vowels.contains(first_char) {
            let new_word = format!("{}{}", word, "-hay");
            new_str = format!("{} {}", new_str, new_word);
        } else {
            let mut word = word.to_string();
            let first = word.remove(0);
            let new_word = format!("{}{}{}{}", word, "-", first, "ay");
            new_str = format!("{} {}", new_str, new_word);
        }
    }
    println!("{new_str}");
}



