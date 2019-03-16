extern crate glob;
extern crate markov;
extern crate rand;

use std::path::{PathBuf};
use markov::Chain;
use glob::glob;

use rand::thread_rng;
use rand::seq::SliceRandom;


fn main() {
    let actor_file = choose_actor();
    let actor = actor_file.display().to_string().replace("corpus/", "").replace(".txt", "");
    
    for s in markov_gen(actor_file) {
        
        println!("[{}] {}",
            actor,
            s
        );
    }
}

fn choose_actor() -> PathBuf {
    let files: Vec<PathBuf> = glob("corpus/*.txt").unwrap().filter_map(Result::ok).collect();
    let mut rng = thread_rng();
    let actor = files.choose(&mut rng).expect("the stage is not set");
    actor.to_owned()
}

fn markov_gen(actor: PathBuf) -> Vec<String>{
    let count = 1;
    
    // Lower order: more random crazy stuff
    // Higher order: more likely to match original line (will need to strip text of punctuation - sentences on new lines)
    
    // TODO: Strip </font>
    // TODO: Strip opening/closing brackets that aren't a pair
    
    let mut chain = Chain::of_order(2);
    chain.feed_file(actor).unwrap();
    if chain.is_empty() { panic!("No files were fed into the chain.") }
    
    chain.str_iter_for(count).collect()
}
