// Functions - Used to store blocks of code for re-use

pub fn run(){
    greeting("Olá", "Renan");

    //bind function values to variables
    let get_sum = add(5,15);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3,3));
}

fn greeting (greet: &str, name: &str) {
 println!("{} {} legal conhecer você!", greet, name);
}

fn add(n1: i32, n2:i32) -> i32 {
    n1 + n2
}