fn main() {
    let to_be_sliced: Vec<i32> = vec![1,2,3,4,5];
    let sliced: Vec<i32> = slice_and_dice(to_be_sliced);
    println!("{:?}", sliced);
}

fn slice_and_dice(input: Vec<i32>) -> Vec<i32> {
    let sliced: Vec<i32> = (&input[1..3]).to_vec(); //takes elements 1 up to but NOT including 3
    sliced
}
