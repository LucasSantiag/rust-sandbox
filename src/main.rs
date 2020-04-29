mod print;
mod variables;
mod types;
mod strings;
mod tuples;
mod arrays;

fn main() {
    println!("Hello, world!");
    print::run();
    variables::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
}
