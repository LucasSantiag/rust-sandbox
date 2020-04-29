pub fn run() {
    println!("Hello from print.rs file");

    println!("Number: {}", 1);

    println!("{} is learning {}", "Lucas", "Rust");

    println!("Hello {0} from {0} code in {1}", "Lucas", "Rust");

    println!("{name} likes to eat {food}", name = "Lucas", food = "banana");

    println!("Binary: {:b}, Hex: {:x}, Oct: {:o}", 10, 10, 10);

    println!("{:?}", (true, 12, "Hello"));

    println!("10 + 10 = {}", 10 + 10);
}
