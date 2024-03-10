use rand::prelude::*;

fn main() {
    println!("Hello, world!");
}

enum Dice {
    D4,
    D6,
    D8,
    D10,
    D20,
}

enum Token {
    Roll(Dice),
    Mult(f64, f64),
    Divide(f64, f64),
    Add(f64, f64),
    Minus(f64, f64),
    Pow(f64, f64),
    Mod(f64, f64),
}
