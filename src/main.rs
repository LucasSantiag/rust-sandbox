mod print;
mod variables;
mod types;
mod strings;
mod tuples;

fn main() {
    println!("Hello, world!");
    print::run();
    variables::run();
    types::run();
    strings::run();
    tuples::run();
}
