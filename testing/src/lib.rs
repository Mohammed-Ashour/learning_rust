#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_contain(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
    fn calc_area(&self) -> u32 {
        return self.width * self.height;
    }
}

#[cfg(test)]
mod tests {
    //importing everything to use inside the module
    use super::*;
    #[test]
    fn test_can_contain() {
        let rect1 = Rectangle {
            width: 32,
            height: 13,
        };
        let rect2 = Rectangle {
            width: 20,
            height: 10,
        };
        assert!(rect1.can_contain(&rect2));
        assert!(!rect2.can_contain(&rect1));
    }
    #[test]
    fn test_calc_area() {
        let rect1 = Rectangle {
            width: 10,
            height: 5,
        };
        assert_eq!(
            rect1.calc_area(),
            50,
            "This msg will only be printed if the test fails, you can use it as a panic msg"
        );
    }
    //adding should_panic will test the code if it panics, you can use it in failures test
    //using expected is optional, it's used to test specific panics if you have multiple of them
    #[test]
    #[should_panic(expected="Failure....")]
    fn failing_test() {
        panic!("Failure....");
    }
}
