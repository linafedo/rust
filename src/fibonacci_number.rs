
pub fn get_fibonacci_number(index: u32, run: bool) {
    if run {
        let mut pair = [0, 0];
        let mut item: u32 = 0;

        for _i in 0..index {
            let first = pair.first().unwrap_or(&0);
            let last = pair.last().unwrap_or(&0);
            item = first + last;
            pair = [*last, if item == 0 { 1 } else { item }];
        }
        println!("Fibonacci value is {item} for index {index}");
    }
}