use std::env;
use std::fs;
use std::{thread, time};


#[derive(Debug, Clone, PartialEq)]
struct IdRange {
}

struct State {
}

impl State {
}

fn main() {
    let file_name = env::args().nth(1).unwrap(); 
    let content: Vec<u8> = fs::read(file_name).unwrap();
    let content_string: String = String::try_from(content).unwrap();

    let example_bank = "818181911112111";

    get_max_joltage(example_bank);
    //println!("answer 1: {}", state.total_added);
    //println!("answer 2: {}", state.total_added);
}

fn get_max_joltage(bank_str: &str) -> u32 {
    let bank = get_bank(bank_str);

    return 0;
}

#[derive(Debug, Clone, Copy)]
struct Battery {
    pos: u32,
    joltage: u32,
}
struct Bank {
    batteries: Vec<Battery>,
}
fn get_bank(bank_str: &str) -> Bank {
    let mut bank = Bank {
        batteries: Vec::new(),
    };
    for (pos, joltage_b) in bank_str.bytes().enumerate() {
        if joltage_b != b'0'..b'9'{
            continue;
        }
        let battery = Battery{
            pos: u32::try_from(pos).unwrap(),
            joltage: u32::from_ascii(joltage_b).unwrap_or(continue),
        };
       bank.batteries.push(battery.clone());
    }

    return bank;
}


