pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    numbers[2] = 1;

    println!("{:?}", numbers);

    numbers.push(12);
    numbers.push(13);
    
    numbers.pop();

    println!("{:?}", numbers);

    for x in numbers.iter() {
        println!("{}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }
    
    println!("{:?}", numbers);
}