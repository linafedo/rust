
#[derive(Debug)]
pub struct Size {
    pub height: i32,
    pub width: i32
}

impl Size {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}

pub fn print_area_for_size(size: &Size, run: bool) {
    if run {
        println!("The area of the rectangle is {} square pixels. ", size.area());
    }
}