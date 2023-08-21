// used to create custom data types

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {

    let mut p = Person::new("Mar", "Brabo");
    println!("Pessoa: {}", p.full_name());
    p.set_last_name("Fraco");
    println!("Pessoa: {} {}", p.first_name, p.last_name);
    println!("Pessoa Tuple: {:?}", p.to_tuple());


    //     let mut c = Color{
    //         red: 255,
    //         green: 0,
    //         blue: 0,
    //     };

    //     c.red = 200;

    //     println!("Color: {} {} {}", c.red, c.green, c.blue);
    // struct Color(u8, u8, u8);
    // let mut c = Color(255, 0, 0);
    // c.0 = 200;
    // println!("Color: {} {} {}", c.0, c.1, c.2);
    // struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

}
