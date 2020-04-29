pub fn run() {
    let x = 1; //default -> i32

    let y = 2.5; //default -> f64

    let z: i64 = 231423132;

    println!("x = {}, y = {}, z = {}", x, y, z);

    println!("Max i32: {}", std::i32::MAX);

    //i32 vs u32 -> u.OnlyPositiveNumber

    let is_active = true;

    let is_greater: bool = 5 > 10;

    let _char = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, _char, face))
}