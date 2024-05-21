fn main() {
    let mut v = vec![6,2,7,1,2,0,2];
    let val: i32 = 2;
    // To remove ONE element:
    // Removes first occurence of it
    v.remove(v.iter().position(|x| *x == val).unwrap());
    println!("{:?}",v);
    // [6, 7, 1, 2, 0, 2]

    // To remove ALL OCCURENCES of an element
    v.retain(|x| *x != val);
    println!("{:?}",v);
    // [6, 7, 1, 2, 0, 2]
}