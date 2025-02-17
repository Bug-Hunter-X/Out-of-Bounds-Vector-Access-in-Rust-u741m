fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // Safe way to access elements: Option<T>
    match vec.get(2) {
        Some(third) => println!("The third element is {}.", third),
        None => println!("There is no third element."),
    }

    //Check index bounds
    if vec.len() > 2 {
        let third = vec[2];
        println!("The third element is {}.", third);
    }
}