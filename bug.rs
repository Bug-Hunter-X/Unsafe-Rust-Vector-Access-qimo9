fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 1;
    // This will panic if the index is out of bounds
    let value = unsafe { *vec.get_unchecked(index) };
    println!("The value at index {} is: {}", index, value);
}