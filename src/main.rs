mod token;
// mod scanner;

use std::env;
use std::fs;
// use token::TokenStream;
// use scanner::Scanner;

fn main() {
    // read command line inputs
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(args[1].clone()).unwrap();
    
    assert_eq!("\"Hello World\"", contents);
}
