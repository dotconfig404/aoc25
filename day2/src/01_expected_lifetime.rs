use std::env;
use std::fs;
use std::{thread, time};


#[derive(Debug, Clone, PartialEq)]
struct IdRange {
    start: &str,
    end: &str,
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
        let id_start: &str = ids.next().unwrap();
        let id_end: &str = ids.next().unwrap();
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
        state.idx += 1;
        
    }

    //println!("answer 1: {}", state.zeroes);
    //println!("answer 2: {}", state.total_zeroes);
}



