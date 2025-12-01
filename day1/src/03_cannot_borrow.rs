use std::env;
use std::fs;

// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// https://doc.rust-lang.org/rust-by-example/trait/clone.html
#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
}

// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
#[derive(Debug, Clone)]
struct Dial {
   dir: Direction,
   len: u32,
}

struct State {
    dials: Vec<Dial>, 
    idx: usize,
    num: u32,
}

impl State {
    fn construct_and_add_dial(&mut self, dir: Direction, len: u32) {
        let dial = Dial {
            dir: dir,
            len: len,
        };
        self.dials.push(dial);
    }

    fn dial_next(&mut self) {
        let mut n = 0; 
        if self.dials[self.idx].dir == Direction::Left {
            n = 100 - self.dials[self.idx].len
        } else {
            n = self.dials[self.idx].len;
        }
        self.num = (n + self.num) % 100;
    }
}

fn main() {
    // args gives back an iterator with Strings. 
    // nth is an iterator function that returns an Option with a Self::Item
    let file_name = env::args().nth(1).unwrap(); 
    //println!("{file_name}");
    let content = get_file_content(&file_name);
    //println!("{content}");
    

    // STATE CONSTRUCTION
    let mut state = State {
        dials: vec![],
        idx: 0,
        num: 50,
    };
    let mut dial_len: String = "".to_string();
    let mut dial_dir: Direction = Direction::Right;
    for c in content.into_iter() {
        match c as char {
            // actually we could also use b'L' etc
            // https://doc.rust-lang.org/reference/tokens.html#byte-literals
            'L' => dial_dir = Direction::Left,
            'R' => dial_dir = Direction::Right,
            // here we could also use b'0'..b'9' ??
            // https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#matching-ranges-of-values-with-..=
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                dial_len.push(c as char);
            }
            _ => {
                state.construct_and_add_dial(dial_dir.clone(), dial_len.parse().unwrap());
                dial_len = "".to_string()
            },
        }
    }
    state.construct_and_add_dial(dial_dir.clone(), dial_len.parse().unwrap());


    // STATE PROCESSING
    for _ in &mut state.dials {
        state.dial_next();    
        print!("SHIT it's {}  ", state.num);
    }
}


fn get_file_content(file_name: &String) -> Vec<u8> {
    // yea, let's just assume we can always find the file!
    return fs::read(file_name).unwrap();
}
