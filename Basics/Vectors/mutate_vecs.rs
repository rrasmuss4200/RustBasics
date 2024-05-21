fn main() {
    let vec: Vec<i32> = vec![1,2,3];
    println!("{:?}", vec_loop(vec));
}

//Multiplies all elements by 2
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        *element *= 2;
    }
    v
}