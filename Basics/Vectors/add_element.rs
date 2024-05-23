fn main() {
    let mut vec: Vec<&str> = vec!["hi", "hello"]; //MUST be mutable to push
    println!("{:?}",vec);
    vec.push("greetings");
    vec.push("salut");
    println!("{:?}",vec);
}