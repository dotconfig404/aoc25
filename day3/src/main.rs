#![allow(unused)]
use std::env;
use std::fs;
use std::cmp::Ordering;

fn main() {
    //let file_name = env::args().nth(1).unwrap(); 
    //let content: Vec<u8> = fs::read(file_name).unwrap();
    //let content_string: String = String::try_from(content).unwrap();

    let example_bank = "818181911112111";

    get_max_joltage(example_bank);
    //println!("answer 1: {}", state.total_added);
    //println!("answer 2: {}", state.total_added);
}

fn get_max_joltage(bank_str: &str) -> u32 {
    let max_joltage = 0;
    let mut bank = get_bank(bank_str);
    sort_bank_by_joltage(&mut bank);
    println!("Sorted: {} ",bank );
    bank.batteries.sort_by_key(|k| k.pos);  
    println!("Sorted: {} ",bank );
    return max_joltage;
}

#[derive(Debug, Clone, Copy)]
struct Battery {
    pos: u32,
    joltage: u32,
}
impl Ord for Battery {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.joltage < other.joltage {
            Ordering::Less
        } else if self.joltage > other.joltage {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Battery{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Battery {
    fn eq(&self, other: &Self) -> bool {
        self.joltage == other.joltage
    }
}
impl Eq for Battery {}


struct Bank {
    batteries: Vec<Battery>,
}
//use std::fmt;
//impl fmt::Display for Bank {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        for b in self.batteries {
//            write!(f, "joltage {} at {}", b.joltage, b.pos)
//        }
//    }
//}
use std::fmt;

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.batteries {
            writeln!(f, "joltage {} at {}", b.joltage, b.pos)?;
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
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                let battery = Battery{
                    pos: pos as u32,//u32::try_from(pos).unwrap(),
                    // int_from_ascii not 'stabilized' yet! it is merged, but seems we must tell
                    // the compiler explicitly that we want to use a unstable library
                    //joltage: u32::from_ascii(&[joltage_b]).unwrap_or(continue),
                    joltage: joltage_c as u32 - '0' as u32,
                    // thx SO https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1
                };
                bank.batteries.push(battery.clone());
                println!("{joltage_c}");
            },
            _ => continue,
        }
    }
    println!("{bank}");
    return bank;
}

fn sort_bank_by_joltage(bank: &mut Bank) {
    bank.batteries.sort(); 
    return;
}

