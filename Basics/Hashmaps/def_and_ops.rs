use std::collections::HashMap;
fn main() {
fn make_parade() -> HashMap<String, u32> {
    let mut parade: HashMap<String,u32> = HashMap::new();
    //add key-value pars
    parade.insert(String::from("Floats"), 8);
    parade.insert(String::from("Balloons"), 500);
    parade.insert(String::from("Marching Bands"), 15);
    parade
}
let parade: HashMap<String, u32> = make_parade();
let parade_type_num: usize = parade.keys().len(); //workds with .values() as well

println!("The parade has objects of {} different kinds!", parade_type_num);
}