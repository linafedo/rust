#[derive(Debug)]
pub struct Rectangle {
    height: u32,
    weight: u32
}

impl Rectangle {
    pub fn new(height:u32, weight: u32) -> Rectangle {
        Rectangle{
            height,
            weight
        }
    }

    pub fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.height > rectangle.height && self.weight > rectangle.weight
    }

    fn is_square(&self) -> bool  {
        self.weight == self.height
    }
 }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let rectangle = Rectangle::new(33,44);
        let rectangle_2 = Rectangle::new(32,40);
        assert!(rectangle.can_hold(&rectangle_2));
    }

    #[test]
    fn smaller_cannt_hold_larger() {
        let rectangle = Rectangle::new(33,44);
        let rectangle_2 = Rectangle::new(40,50);
        assert!(!rectangle.can_hold(&rectangle_2));
    }

    #[test]
    fn is_equal() {
        let rectangle = Rectangle::new(33,33);
        assert_eq!(rectangle.is_square(), true);
    }

    #[test]
    #[ignore]
    fn is_ne() {
        let rectangle = Rectangle::new(33,33);
        assert_ne!(rectangle.is_square(), false);
    }
}
