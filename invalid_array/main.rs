use std::io;

fn main() {
    let a = [1,2,3,4,5];

    println!("Please enter an index:");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.
        trim().
        parse().
        expect("Failed to parse index");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}