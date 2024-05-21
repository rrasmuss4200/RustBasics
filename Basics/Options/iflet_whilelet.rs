fn main() {
    //Option is either Some or None
    let scoops: u32 = 5;

    let optional_target: Option<u32> = Some(scoops);


    // IF LET
    // Means if optional_target is type Some,
    // then number will be assigned to the value inside optional_target
    if let Some(number) = optional_target {
        assert_eq!(number,scoops);
    }

    // WHILE LET
    let mut stack = vec![1,2,3];
    // compares to see if the value from stack.pop() is Some. If it is, it continues the loop.
    while let Some(top) = stack.pop() {
        // .pop() returns Some if it reads a value.
        // Returns None once it goes out of range.
        println!("{}",top);
    }

}