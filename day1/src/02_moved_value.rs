use std::env;
use std::fs;

// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
enum Direction {
    Left,
    Right,
}

// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
struct Dial {
   dir: Direction,
   len: u32,
}

fn main() {
    // args gives back an iterator with Strings. 
    // nth is an iterator function that returns an Option with a Self::Item
    let file_name = env::args().nth(1).unwrap(); 
    //println!("{file_name}");
    let content = get_file_content(&file_name);
    //println!("{content}");
    let mut dials: Vec<Dial> = vec![];
    let mut current_dial_dir: Direction = Direction::Right;
    let mut current_dial_len: String = "".to_string();
    for c in content.into_iter() {
        match c as char {
            // actually we could also use b'L' etc
            // https://doc.rust-lang.org/reference/tokens.html#byte-literals
            'L' => current_dial_dir = Direction::Left,
            'R' => current_dial_dir = Direction::Right,
            // here we could also use b'0'..b'9' ??
            // https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#matching-ranges-of-values-with-..=
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                println!("Pushing {}", c as char);
                current_dial_len.push(c as char);
                println!("currnet: {current_dial_len}");
            }
            _ => {
                println!("currnet: {current_dial_len}");
                dials.push(Dial{
                    dir: current_dial_dir,
                    len: current_dial_len.parse().unwrap(),
                    
                });
                current_dial_len = "".to_string()
            },
        }
    }
}


fn get_file_content(file_name: &String) -> Vec<u8> {
    // yea, let's just assume we can always find the file!
    return fs::read(file_name).unwrap();
}
