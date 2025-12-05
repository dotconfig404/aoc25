#![allow(unused)]
use std::env;
use std::fs;
use std::cmp::Ordering;

fn main() {
    let file_name = env::args().nth(1).unwrap(); 
    let content: Vec<u8> = fs::read(file_name).unwrap();
    let content_string: String = String::try_from(content).unwrap();

    let example_bank = "818181911112111";

    //let max_joltage = get_max_joltage(example_bank);
    //println!("answer 1: {}", state.total_added);
    //println!("answer 2: {}", state.total_added);
    let mut max_joltage: u32 = 0;
    let banks = content_string.split('\n');
    for bank in banks {
        println!("{bank}");
        println!("max: {}", get_max_joltage(&bank));
        max_joltage += get_max_joltage(&bank);
    }
    println!("max joltage: {}", max_joltage);
}

fn get_max_joltage(bank_str: &str) -> u32 {
    let mut max_joltage = 0;
    let mut bank = get_bank(bank_str);

    // reverse sorting by joltage -> highest joltage first
    bank.batteries.sort_by(|a, b| b.joltage.cmp(&a.joltage));
    let highest = &bank.batteries[0];
    println!("{bank}");
    let mut bank2 = get_bank(bank_str);
    bank.batteries = bank2.batteries.split_off(highest.pos as usize);
    println!("{bank}");

    // sorting new one
    bank.batteries.sort_by(|a, b| b.joltage.cmp(&a.joltage));
    
    max_joltage = combine_batteries_to_joltage(&bank.batteries[0], &bank.batteries[1]);
        
    return max_joltage;
}

fn combine_batteries_to_joltage(left_bat: &Battery, right_bat: &Battery) -> u32 {
    let mut l_jolt = left_bat.joltage.to_string();
    let r_jolt = right_bat.joltage.to_string();
    l_jolt.push_str(&r_jolt); 
    return l_jolt.parse().unwrap();
}

#[derive(Debug, Clone, Copy)]
struct Battery {
    pos: u32,
    joltage: u32,
}

struct Bank {
    batteries: Vec<Battery>,
}
use std::fmt;

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //writeln!("Joltages");
        for b in &self.batteries {
            write!(f, "{}", b.joltage)?;
        }
        Ok(())
    }
}


fn get_bank(bank_str: &str) -> Bank {
    let mut bank = Bank {
        batteries: Vec::new(),
    };
    for (pos, joltage_c) in bank_str.chars().enumerate() {
        match joltage_c {
            '0'..='9' => {
                let battery = Battery{
                    pos: pos as u32,//u32::try_from(pos).unwrap(),
                    // int_from_ascii not 'stabilized' yet! it is merged, but seems we must tell
                    // the compiler explicitly that we want to use a unstable library
                    //joltage: u32::from_ascii(&[joltage_b]).unwrap_or(continue),
                    joltage: joltage_c as u32 - '0' as u32,
                    // thx SO https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1
                };
                bank.batteries.push(battery.clone());
            },
            _ => continue,
        }
    }
    return bank;
}

//fn sort_bank_by_joltage(bank: &mut Bank) {
//    bank.batteries.sort(); 
//    return;
//}

