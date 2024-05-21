fn main() {
    let mut vec: Vec<&str> = vec!["hi", "hello"]; //MUST be mutable to append
    let mut vector: Vec<&str> = vec!["greetings"];
    vec.append(&mut vector);
    println!("{:?}",vec);
}