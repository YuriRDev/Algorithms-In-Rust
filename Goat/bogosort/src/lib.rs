use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Bogosort {
    vec: Vec<i32>,
    count: u32,
}

impl Bogosort {
    pub fn new(value: Vec<i32>) -> Bogosort {
        Bogosort {
            vec: value,
            count: 0,
        }
    }

    pub fn start(&mut self) {
        while !self.is_ordered() {
            self.count += 1;
            self.rand();
        }

        println!("It took: {} \"randomizations\": {:?}", self.count, self.vec);
    }

    fn rand(&mut self) {
        self.vec.shuffle(&mut thread_rng());
    }

    fn is_ordered(&self) -> bool {
        for _b in 1..self.vec.len() {
            if self.vec[_b - 1] > self.vec[_b] {
                return false;
            }
        }
        true
    }
}
