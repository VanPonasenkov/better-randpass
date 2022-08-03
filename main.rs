use rand::*;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

struct Config {
    is_numbers: bool,
    is_uppercase: bool,
    is_lowercase: bool,
    range: u16,
}

#[inline(always)]
fn is_yes(string: &String) -> bool {
    return string == "yes" || string == "y";
}

fn generate_random_password(config: Config) -> String {
    let mut password = String::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    );
    let mut chars_from = String::new();
    if config.is_numbers {
        chars_from.push_str("1234567890");
    }
    if config.is_uppercase {
        chars_from.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if config.is_lowercase {
        chars_from.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    let (chars_from, chars_from_len) = (chars_from.as_bytes(), chars_from.len());
    for _ in 0..config.range {
        let idx: usize = rng.gen_range(0..chars_from_len);
        password.push(chars_from[idx] as char);
    }
    return password;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_is_number = args
        .get(1)
        .expect("Expected 'is_number' argument in position 1");
    let args_isupper = args
        .get(2)
        .expect("Expected 'is_uppercase' argument in position 2");
    let args_islower = args
        .get(3)
        .expect("Expected 'is_lowercase' argument in position 3");
    let args_range = args
        .get(4)
        .expect("Expected 'range' argument in position 4")
        .parse::<u16>()
        .expect("'range' argument is not a number");

    if args_range <= 0 {
        panic!("Range must be greater than 0");
    }

    let config = Config {
        is_numbers: is_yes(&args_is_number),
        is_uppercase: is_yes(&args_isupper),
        is_lowercase: is_yes(&args_islower),
        range: args_range,
    };
    println!("{}", generate_random_password(config));
}
