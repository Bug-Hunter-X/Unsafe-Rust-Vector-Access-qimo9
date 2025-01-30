fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 1;
    // Safe way to access vector elements
    match vec.get(index) {
        Some(value) => println!("The value at index {} is: {}", index, value),
        None => println!("Index out of bounds"),
    };
}