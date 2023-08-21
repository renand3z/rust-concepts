// #![allow(unused)]
pub fn run() {

pub struct Guess {
    value: i32,
}

impl Guess {
// can only accept a guess struct on this conditions
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
// borrow the value and return something
    pub fn value(&self) -> i32 {
        self.value
    }
}

let hey1 = Guess::new(8);
let hey2 = Guess::value(&hey1);

// let hey2 = Guess::value(8);
// println!("{}", hey1);
println!("{}", hey2);
}