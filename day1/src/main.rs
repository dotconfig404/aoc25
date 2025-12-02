use std::env;
use std::fs;
use std::{thread, time};

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
   len: i32,
}

struct State {
    dials: Vec<Dial>, 
    idx: usize,
    num: i32,
    zeroes: u32,
    total_zeroes: u32,
}

impl State {
    fn construct_and_add_dial(&mut self, dir: Direction, len: i32) {
        let dial = Dial {
            dir: dir,
            len: len,
        };
        self.dials.push(dial);
    }

    fn dial_next(&mut self) {
        //let mut n = 0; 
        //if self.dials[self.idx].dir == Direction::Left {
        //    n = -self.dials[self.idx].len
        //} else {
        //    n = self.dials[self.idx].len;
        //}
        //self.idx += 1;
        //println!("Iteration: {}\n Zeroes: {}, TotalZeroes: {}", self.idx,self.zeroes, self.total_zeroes);
        //println!("Initial: Num: {}, n: {}", self.num, n);
        ////self.num += n;
        //println!("Postadd: Num: {}, n: {}", self.num, n);

        let mut l = self.dials[self.idx].len;
        let mut d = self.dials[self.idx].dir.clone();
        while l != 0 {
            if  d == Direction::Left {
                if self.num - l > 0{
                    self.num += l;
                } else if l < -99 {
                    l += 99;
                    self.total_zeroes += 1;
                } else {
                    l = 0;
                    self.total_zeroes += 1;
                    self.zeroes += 1;
                }
            }
            if  d == Direction::Right {
                if self.num + l < 100 {
                    self.num += l;
                } else if l > 99{
                    l -= 100;
                    self.total_zeroes += 1;
                } else {
                    l = 0;
                    self.total_zeroes += 1;
                    self.zeroes += 1;
                }
            }
            // add n if not going above 100
            //if n > 0 && n + self.num <= 99 {
            //    self.num += n; 
            //    n = 0;
            //// else subtract
            //} else if n > 0 {
            //    n -= 100;
            //    self.total_zeroes +=1;
            //} 
            //if n < 0 && n - self.num >= 0 {
            //    self.num -= n;
            //    n = 0;
            //} else if n < 0 {
            //    n += 100;
            //    self.total_zeroes += 1;
            //}

            //if self.num > 99 {
            //    self.num -=100;
            //    self.total_zeroes += 1;
            //} else if self.num < 0 {
            //    self.num += 100;
            //    self.total_zeroes += 1;
            //}
            //if self.num == 0 {
            //    self.total_zeroes += 0;
            //    self.zeroes += 0;
            //}
        }
        //while self.num >= 100 {
        //    thread::sleep(time::Duration::from_millis(100));
        //    println!("Too High: Num: {}, n: {}", self.num, n);
        //    self.total_zeroes += 1;
        //    self.num -= 100;
        //}

        //while self.num < 0 {
        //    thread::sleep(time::Duration::from_millis(100));
        //    println!("Too Low: Num: {}, n: {}", self.num, n);
        //    self.total_zeroes += 1;
        //    self.num += 100;
        //}
        //if self.num == 0 {
        //    self.zeroes += 1;
        //}
        //println!("Final: Num: {}, n: {}", self.num, n);
        //println!("Done. Zeroes: {}, TotalZeroes: {}\n", self.zeroes, self.total_zeroes);

        //6504 too high (so is 6503)
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
        zeroes: 0,
        total_zeroes: 0,
    };
    let mut dial_len: String = "".to_string();
    let mut dial_dir: Direction = Direction::Right;
    for c in content.into_iter() {
        match c as char {
            // actually we could also use b'L' etc
            // https://doc.rust-lang.org/reference/tokens.html#byte-literals
            // https://doc.rust-lang.org/book/appendix-02-operators.html#non-operator-symbols
            'L' => dial_dir = Direction::Left,
            'R' => dial_dir = Direction::Right,
            // here we could also use b'0'..b'9' ??
            // https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#matching-ranges-of-values-with-..=
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                dial_len.push(c as char);
            }
            '\n' => {
                if ! dial_len.is_empty() {
                    state.construct_and_add_dial(dial_dir.clone(), dial_len.parse().unwrap());
                    dial_len = "".to_string()
                }
            },
            _ => (),
        }
    }


    // STATE PROCESSING
    for _ in 0..state.dials.len() {
        state.dial_next();    
    }
    println!("answer 1: {}", state.zeroes);
    println!("answer 2: {}", state.total_zeroes);
}


fn get_file_content(file_name: &String) -> Vec<u8> {
    // yea, let's just assume we can always find the file!
    return fs::read(file_name).unwrap();
}
