use std::env;
use std::fs;

fn main() {
    // args gives back an iterator with Strings. 
    // nth is an iterator function that returns an Option with a Self::Item
    let file_name = env::args().nth(1).unwrap(); 
    //println!("{file_name}");
    let content = get_file_content(&file_name);
    //println!("{content}");
}

fn get_file_content(file_name: &String) -> Vec<u8> {
    return fs::read(file_name)?;
}
// error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
//   --> src/main.rs:15:31
//    |
// 14 | fn get_file_content(file_name: &String) -> Vec<u8> {
//    | -------------------------------------------------- this function should return `Result` or `Option` to accept `?`
// 15 |     return fs::read(file_name)?;
//    |                               ^ cannot use the `?` operator in a function that returns `Vec<u8>`

// the questionmark operator is convenient if several functions are used that return a Result. In
// that case we can use it to automatically either return an Err or just unwrap the value inside Ok
// https://doc.rust-lang.org/std/result/index.html#the-question-mark-operator-
// 
