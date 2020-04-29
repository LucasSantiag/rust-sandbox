pub fn run() {
    let age: u8 = 18;
    let check_id: bool = true;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 18 && check_id {
        println!("Bartender: Sorry, but you cannot drink");
    } else {
        println!("Bartender: Can i check your id?");
    }

    let is_under_age = if age < 18 {true} else {false};
    println!("Is under age: {}", is_under_age);
}