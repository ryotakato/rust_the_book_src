pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(left: u64) -> u64 {
    left + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn it_works() {
    //    let result = add(2, 2);
    //    assert_eq!(result, 4);
    //}

    //#[test]
    //fn exploration() {
    //    let result = add(4, 4);
    //    assert_eq!(result, 9);
    //}

    //#[test]
    //fn another() {
    //    panic!("Make this test fail");
    //}
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller), "`{:?} is needed to be able to include `{:?}", larger, smaller);
    }

    //#[test]
    //fn smaller_cannot_hold_larger() {
    //    let larger = Rectangle {
    //        width: 8,
    //        height: 7
    //    };

    //    let smaller = Rectangle {
    //        width: 5,
    //        height: 1
    //    };

    //    assert!(!smaller.can_hold(&larger));
    //}

    //#[test]
    //#[should_panic(expected = "Guess value must be less than or equal to 100")]
    //fn greater_than_100() {
    //    Guess::new(200);
    //}

}
