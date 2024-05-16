// derived from rustlings example
#[derive(PartialEq, Debug)]

struct PositiveNonzeroInteger(u64); // tuple struct

#[derive(PartialEq, Debug)]
// Custom error to return
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value < 0 {
            Err(CreationError::Negative)
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

fn main() {
    // .unwrap() stops the program if an error is returned
    let x = PositiveNonzeroInteger::new(10).unwrap(); // no stop
    let y = PositiveNonzeroInteger::new(-5).unwrap(); // stops program
    format!("{x:?} {y:?}");
}
