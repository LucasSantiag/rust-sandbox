pub fn run() {
    let arr = [1, 2, 3];
    let other_arr = arr;

    println!("Values: {:?}", (arr, other_arr));

    let vec = vec![1, 2, 3];
    let other_vec = &vec;

    println!("Values: {:?}", (&vec, other_vec));
}