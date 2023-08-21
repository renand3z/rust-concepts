pub fn run() {
    let age: u16 = 16;
    let check_id: bool = true;
    let knows_person_of_age = true;

        if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: Boa noite, vai beber o que?");
    } else if age < 21 && check_id {
        println!("Bartender: De menor aqui não!");
    } else {
        println!("Bartender: Cadê a identidade?");
    }

let is_of_age = if age >= 21 { true } else { false }
;
println!("Is of age {}", is_of_age)

}    
