// Result type returns either Ok() or Err()
fn main() {
    pub fn make_cereal(milk: bool) -> Result<String, String> {
    if !milk {
        Err(String::from("You need milk for your cereal!"))
    } else {
        Ok(String::from("Enjoy your yummy snack."))
    }
}
let milk: bool = true;
let _ = make_cereal(milk);
println!("{:?}",make_cereal(milk));
}