use std::env;
use std::fs;
use std::{thread, time};


#[derive(Debug, Clone, PartialEq)]
struct IdRange {
    start: u32,
    end: u32,
    invalid_ids: Vec<u32>,
    added: u32,
}

struct State {
    id_ranges: Vec<IdRange>, 
    idx: usize,
    total_added: u32,
}

impl State {
    fn add_id_range(&mut self, range: IdRange) {
        self.id_ranges.push(range);
    }
}

fn main() {
    let file_name = env::args().nth(1).unwrap(); 
    let content: Vec<u8> = fs::read(file_name).unwrap();
    let content_string: String = String::try_from(content).unwrap();

    // https://doc.rust-lang.org/std/string/struct.String.html#method.split
    let mut content_split = content_string.split(',');

    let mut state = State {
        id_ranges: Vec::<IdRange>::new(),
        idx: 0,
        total_added: 0,
    };

    for id_range in content_split {
        // last new line cant be parsed to int
        let id_range = id_range.trim();
        let mut ids = id_range.split('-');
        let id_start: u32 = ids.next().unwrap().parse().unwrap();
        let id_end: u32 = ids.next().unwrap().parse().unwrap();
        println!("{id_start}, {id_end}");
        let range = IdRange{
            start: id_start,
            end: id_end,
            invalid_ids: Vec::<u32>::new(),
            added: 0,
        };
        state.add_id_range(range.clone());
        //state.put_id_range(id_start, id_end);
    }

    // STATE PROCESSING
    for id_range in state.id_ranges {
        println!("{}, {}, {}", state.idx, id_range.start, id_range.end);
        //let mut num = id_range.start;
        for num in id_range.start..id_range.end+1 {
            println!("num: {}", num);
            let num_len = num.to_string().len();
            if num_len % 2 == 0 {
               let (n1, n2) = num.to_string().split_at(num_len/2);
               println!("{n1}, {n2}");
            }
        }
        state.idx += 1;
    }

    //println!("answer 1: {}", state.zeroes);
    //println!("answer 2: {}", state.total_zeroes);
}



