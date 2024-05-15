fn main() {
fn add_bar(x: Option<String>) -> Option<String> {
    match x {
        None => None,
        Some(mut x) => {
            x.push_str("bar"); //push_str returns (). return needs
            Some(x)             //to be done in two steps
        }
    }
}
let string = Some("hi".to_string());
let new_string = add_bar(string);
println!("{:?}", new_string); //Some("hibar")
}