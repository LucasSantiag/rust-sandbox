//max 12 elements
pub fn run() {
    let person: (&str, &str, i32) = ("Lucas", "Brazilian", 18);

    println!("Hi, my name is {} and I am a {} years old {}", person.0, person.1, person.2)
}