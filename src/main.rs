#![windows_subsystem = "windows"]

use rand::prelude::*;

#[derive(Debug)]
enum AppError {
    ParseIntError(std::num::ParseIntError),
    Error(std::io::Error),
    ErrorMessage(String),
}

fn gen_pass(params: Vec<&str>) -> Result<String, AppError> {
    let mut allowed_chars_type: Vec<u8> = Vec::new();
    let mut _custom_symbols: Vec<char> = Vec::new();

    // 0-numbers, 1-upper, 2-lower, 3-symbols, 4-custom_symbols
    for c in params[1].chars().into_iter() {
        match c {
            '0' => allowed_chars_type.push(0),
            '1' => allowed_chars_type.push(1),
            '2' => allowed_chars_type.push(2),
            '3' => allowed_chars_type.push(3),
            '4' => {
                allowed_chars_type.push(4);
                _custom_symbols = match parse_new_symbols() {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(AppError::Error(e));
                    }
                };
            }
            _ => {
                return Err(AppError::ErrorMessage(
                    "Not allowed character type, use 0-4 instead".to_string(),
                ))
            }
        }
    }

    // Parsing the lenght of the future password
    let mut password = String::new();
    let mut counter = match params[2].parse::<u8>() {
        Ok(n) => n,
        Err(err) => return Err(AppError::ParseIntError(err)),
    };

    // Applying and generating random char type for every character in future password
    let mut rng = thread_rng();
    while counter != 0 {
        // rand choose of char type
        let rand_idx = rng.gen_range(0..allowed_chars_type.len());
        match allowed_chars_type[rand_idx] {
            0 => {
                let char = char::from_u32(rng.gen_range(48..=57)).unwrap();
                password.push(char);
            }
            1 => {
                let char = char::from_u32(rng.gen_range(65..=90)).unwrap();
                password.push(char);
            }
            2 => {
                let char = char::from_u32(rng.gen_range(97..=122)).unwrap();
                password.push(char);
            }
            3 => {
                let char = char::from_u32(rng.gen_range(33..=47)).unwrap();
                password.push(char);
            }
            4 => {
                if _custom_symbols.len() != 0 {
                    let char = _custom_symbols[rng.gen_range(0.._custom_symbols.len())];
                    password.push(char);
                }
            }
            _ => {
                continue;
            }
        }
        counter -= 1;
    }

    // Test enthropy, where lower+upper+numbers = 60
    let enthropy = (60.0 + _custom_symbols.len() as f32).log2() * params[2].parse::<f32>().unwrap();
    match enthropy as u16 {
        0..=75 => println!("Weak password: {}", enthropy),
        76..=100 => println!("Normal password: {}", enthropy),
        101..=120 => println!("Strong password: {}", enthropy),
        121.. => println!("Very strong, exactly what you need: {}", enthropy),
    }
    Ok(password)
}

// Getting new characters from user through console
fn parse_new_symbols() -> Result<Vec<char>, std::io::Error> {
    println!("Enter new characters bellow without spaces between");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input)?;

    Ok(user_input.trim().chars().into_iter().collect::<Vec<char>>())
}

fn windows_console_block() {
    println!("Press any key to close this program...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn main() -> Result<(), std::io::Error> {
    println!("Welcome, to use this program write 'run <allowed code> <password lenght>'");
    println!("example: run 023 34, to stop write 'stop'");
    println!("//allowed codes: 0-numbers, 1-upper, 2-lower, 3-symbols, 4-custom_symbols");

    loop {
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input)?;
        let mut _params: Vec<&str> = Vec::new();

        if user_input.trim() == "stop" {
            break;
        }

        _params = user_input.trim().splitn(3, ' ').collect::<Vec<&str>>();

        if _params.len() == 3 {
            match gen_pass(_params) {
                Ok(password) => println!("{}", password),
                Err(err) => println!("{:?}", err),
            }
            continue;
        } else {
            println!("Please enter a valid command");
        }
        println!("Enter a command bellow");
    }
    windows_console_block();
    Ok(())
}
