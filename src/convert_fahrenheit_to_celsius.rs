
const SUBTRAHEND:i32 = 32;
const DIVIDER:f32 = 1.8;

pub fn convert_fahrenheit_to_celsius(item: f32, run: bool) {
    if run {
        let degrees = (item - SUBTRAHEND as f32) / DIVIDER;
        println!("{item} degrees Fahrenheit equals {degrees} degrees Celsius");
    }
}