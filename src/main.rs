pub mod syntatic_analyzer;

#[cfg(test)]
mod tests; 

use std::env;
use std::fs;

fn main() {
    // read command line inputs
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(args[1].clone()).unwrap();
    
    assert_eq!("\"Hello World\"", contents);
}
