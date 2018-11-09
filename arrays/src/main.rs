fn main() {
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;

    //assert_eq!([1, 2], &array[1..]);

    // This loop prints: 0 1 2
    for x in array.iter() {
        print!("{} ", x);
    }   
    println!();
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
