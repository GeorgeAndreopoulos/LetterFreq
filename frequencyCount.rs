use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let mut lettersCount: HashMap<char,i32> = HashMap::new();
    let mut beginChar = 'a';
    lettersCount.insert('a',0);
    for i in 1..26{
       beginChar = ((beginChar as u8) + 1) as char;
       lettersCount.insert(beginChar,0);
    }
   //println!("{:?}", lettersCount);

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path);
    println!("{:?}",content);
}
