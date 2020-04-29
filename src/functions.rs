pub fn run() {
    greeting("Hello", "Lucas");
    
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);
    
    let z: i32 = 10;
    let add_nums = |x: i32, y: i32| x + y + z;
    println!("Second sum: {}", add_nums(1, 6));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}