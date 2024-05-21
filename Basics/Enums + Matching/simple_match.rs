enum Numeral {
    I,
    V,
    X,
    L,
}

fn roman_to_int(numeral: Numeral) -> u32 {
    match numeral {
        Numeral::I => 1,
        Numeral::V => 5,
        Numeral::X => 10,
        Numeral::L => 50,
    }
}