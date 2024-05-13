fn main() {
    let mut vec: Vec<&str> = vec!["hi", "hello"];
    let mut vector: Vec<&str> = vec!["greetings"];
    vec.append(&mut vector);
    println!("{:?}",vec);
}