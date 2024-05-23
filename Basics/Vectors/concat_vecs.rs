fn main() {
    let vec1: Vec<i32> = vec![1,2,3,4];
    let vec2: Vec<i32> = vec![5,6,7,8];
    let nums: Vec<i32> = [vec1,vec2].concat();
    println!("{:?}",nums);
}