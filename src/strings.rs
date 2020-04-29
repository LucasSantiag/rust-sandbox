pub fn run() {
    //str vs String
    let _hello = "Hi";
    let mut hello = String::from("Hello ");

    println!("Length: {}", hello.len());

    hello.push('W');

    hello.push_str("orld!");

    println!("Is Empty: {}", hello.is_empty());

    println!("{}", _hello);
    println!("{}", hello);

    let mut s = String::from("");
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
}