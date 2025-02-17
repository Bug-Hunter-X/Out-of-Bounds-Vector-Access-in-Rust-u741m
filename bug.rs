fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let third = vec[2]; //this line will cause a panic because it tries to access an index which is out of bounds
    println!("The third element is {}.", third);
}