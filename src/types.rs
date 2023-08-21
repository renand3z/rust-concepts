pub fn run (){

    let x = 1;

    let y = 2.5;

    let z: i64 = 454545454545454;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater = 10 > 20;

    let a1 = 'a';

    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}