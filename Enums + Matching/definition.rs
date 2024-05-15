//example from rust book
//types of messages
enum Message {
    Quit, //unit struct
    Move {x: i32, y: i32}, //normal struct
    Write(String), //tuple struct
    ChangeColor(u8,u8,u8), //tuple struct
}

impl Message {
    fn call(&self) {
        //body
    }
}

let m = Message::ChangeColor(25,74,200); //assignment
m.call(); //method call