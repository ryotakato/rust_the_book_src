
struct Counter {
    count: u32,
}


impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}


// Counter as a New Type
struct Counter2(u32);

impl Counter2 {
    fn new() -> Counter2 {
        Counter2(0)
    }
}

impl Iterator for Counter2 {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;

        if self.0 < 6 {
            Some(self.0)
        } else {
            None
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn calling_next_directly2() {
        let mut counter = Counter2::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                     .map(|(a, b)| a * b)
                                     .filter(|x| x % 3 == 0)
                                     .sum();
        assert_eq!(18, sum);
    }
}
