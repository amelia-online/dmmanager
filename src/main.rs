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
            _ => {}
        }
    }
}

fn roll(dice: DiceType) -> u32 {
    use DiceType::*;
    let mut rng = rand::thread_rng();
    let lower = 1;
    let mut upper = 20;
    match dice {
        D4 => upper = 4,
        D6 => upper = 6,
        D8 => upper = 8,
        D10 => upper = 10,
        D12 => upper = 12,
        D20 => {}
    }
    rng.gen_range(lower..upper)
}

fn extract_string(index: &mut usize, chars: &Vec<char>) -> Option<String> {
    let mut result = String::new();

    if *index == chars.len() - 1 {
        return None;
    }

    // index starts on a "
    // and it'll end on a "

    *index += 1;
    let mut chr = chars[*index];

    while chr != '\"' {
        if *index == chars.len() - 1 {
            return None;
        }

        chr = chars[*index];

        match chr {
            '\\' => {
                *index += 1;
                if let ch = chars[*index] {
                    match ch {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '\'' => result.push('\''),
                        '\"' => result.push('\"'),
                        _ => return None,
                    }
                } else {
                    return None;
                }
            }
            _ => result.push(chr),
        }

        *index += 1;
    }

    Some(result)
}

fn lex(string: &String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut line_num = 0;
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
            }
            ' ' => {
                index += 1;
            }
            '\"' => {
                if let Some(literal) = extract_string(&mut index, &chars) {
                    result.push(Token::Literal(literal));
                    index += 1;
                } else {
                    report_error(line_num, "Invalid string encountered.");
                }
            }
            _ => {
                buf.push(chr);
                index += 1;
                continue;
            }
        }

        if let Some(token) = process_string(&buf) {
            result.push(token);
        } else {
            report_error(line_num, "Unknown token encountered.");
        }

        buf.clear();
    }

    result
}

fn process_string(string: &String) -> Option<Token> {
    use DiceType::*;
    use Token::*;

    if string.is_empty() {
        return None;
    }

    if let Ok(num) = string.parse::<f64>() {
        return Some(Number(num));
    }

    match string.trim() {
        "d4" | "D4" => Some(Dice(D4)),
        "d6" | "D6" => Some(Dice(D6)),
        "d8" | "D8" => Some(Dice(D8)),
        "d10" | "D10" => Some(Dice(D10)),
        "d12" | "D12" => Some(Dice(D12)),
        "d20" | "D20" => Some(Dice(D20)),
        "roll" => Some(Roll),
        "create" => Some(Create),
        "*" => Some(Mult),
        "/" => Some(Divide),
        "+" => Some(Add),
        "-" => Some(Minus),
        "^" => Some(Pow),
        "%" => Some(Mod),
        "->" => Some(LittleArrow),
        "=>" => Some(BigArrow),

        _ => Some(Ident(string.to_owned())),
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
    Mult,
    Divide,
    Add,
    Minus,
    Pow,
    Mod,
    Ident(String),
    Number(f64),
    Create,
    LittleArrow,
    BigArrow,
    Literal(String),
}
