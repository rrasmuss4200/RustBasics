fn main() {
    fn1(2);
    fn2("string".to_string());
}

fn fn1(input: i32) -> i32{
    println!("This is how you return an int {}", input);
    input //return statement
}

fn fn2(input: String) -> String {
    println!("This is how you return a {}", input);
    input // return statement
}