struct ClassicStruct {
    red: i32,
    green: i32,
    blue: i32,
}

struct TupleStruct(i32, i32, i32);

#[derive(Debug)]
struct UnitTypeStruct;

fn main() {
    // Setting apple.red = 255, apple.green = 0, apple.blue = 0
    let _apple: ClassicStruct = ClassicStruct {
        red: 255,
        green: 0,
        blue: 0,
    };
    //Setting sky.red = 0, sky.green = 0, sky.blue = 255
    let _sky: TupleStruct = TupleStruct(0,0,255);

    let unit: UnitTypeStruct = UnitTypeStruct;
    println!("{:?}s are fun!", unit);
    //Prints name of struct.
    // "UnitTypeStructs are fun!""
}
