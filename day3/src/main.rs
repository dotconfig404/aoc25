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
    let mut max_joltage = 0;
    let mut bank = get_bank(bank_str);
    sort_bank_by_joltage(&mut bank);
    let highest = &bank.batteries[0];
    let next_highest = &bank.batteries[1];
    // if highest voltage is after the next max voltage = 
    // next_highest'+'highest
    if highest.pos > next_highest.pos {
        max_joltage = combine_batteries_to_joltage(&highest, &next_highest);
    }
        
    bank.batteries.sort_by_key(|k| k.pos);
    return max_joltage;
}

fn combine_batteries_to_joltage(bat1: &Battery, bat2: &Battery) -> u32 {
    let mut j1_s = bat1.joltage.to_string();
    let j2_s = bat2.joltage.to_string();
    return j1_s.push(j2_s).parse().unwrap();
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
            '0'..'9' => {
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

