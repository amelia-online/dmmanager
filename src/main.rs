use rand::prelude::*;
use std::io::{self, Write};

fn main() {
    println!("Welcome to Dungeon Master Manager (DMManager)");
    loop {
        let input = get_input();
        let tokens = lex(&input);
        for token in tokens {
            println!("{:?}", token);
        }
        match input.trim() {
            "quit" => break,
            _ => println!("<< {}", input.to_string()),
        }
    }
}

fn lex(string: &String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut line_num = 1;
    let mut index = 0;

    fn report_error(line: u32, msg: &str) {
        println!("[Error] <Line {}>: {}", line, msg);
    }

    let mut buf = String::new();
    let chars: Vec<char> = string.chars().collect();

    while index < chars.len() {
        let chr = chars[index];

        match chr {
            '\n' => {
                line_num += 1;
                index += 1;
                continue;
            }
            ' ' => {
                index += 1;
                if let Some(token) = process_string(&buf) {
                    result.push(token);
                }
                buf.clear();
            }
            _ => {
                buf.push(chr);
                index += 1;
            }
        }
    }

    if let Some(token) = process_string(&buf) {
        result.push(token);
    }
    buf.clear();

    result
}

fn process_string(string: &String) -> Option<Token> {
    use DiceType::*;
    use Token::*;
    match string.trim() {
        "d4" | "D4" => Some(Dice(D4)),
        "d6" | "D6" => Some(Dice(D6)),
        "d8" | "D8" => Some(Dice(D8)),
        "d10" | "D10" => Some(Dice(D10)),
        "d12" | "D12" => Some(Dice(D12)),
        "d20" | "D20" => Some(Dice(D20)),
        "roll" => Some(Roll),

        _ => None,
    }
}

fn get_input() -> String {
    print!("> ");
    io::stdout().flush().expect("");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    input
}

#[derive(Debug)]
enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

#[derive(Debug)]
enum Token {
    Dice(DiceType),
    Roll,
    Mult(f64, f64),
    Divide(f64, f64),
    Add(f64, f64),
    Minus(f64, f64),
    Pow(f64, f64),
    Mod(f64, f64),
    Ident(String),
}
