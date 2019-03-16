extern crate markov;

use std::path::Path;
use markov::Chain;

fn main() {
    for s in markov_gen() {
        println!("{:?}", s);
    }
}

fn markov_gen() -> Vec<String>{
    let count = 1;
    
    // let mut chain = Chain::new();
    let mut chain = Chain::of_order(4);
    chain.feed_file(Path::new("corpus/jerry.txt")).unwrap();
    if chain.is_empty() { panic!("No files were fed into the chain.") }
    
    chain.str_iter_for(count).collect()
}
