pub fn run() {
    let name = "Lucas";
    let mut age = 17;

    /*let age = 19;
     *age = 17; <- do not work.... Imutability */
    
    println!("My name is {} and I am {}", name, age);

    age = 18;        
    
    println!("My name is {} and I am {}", name, age);
}