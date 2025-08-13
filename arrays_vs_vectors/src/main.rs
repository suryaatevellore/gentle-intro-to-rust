fn main() {
    let mut array = [1,2,3];
    for x in array[1..] {
        println!("{x}")
    }
    for x in array {
        println!("{x}");
    }
}
