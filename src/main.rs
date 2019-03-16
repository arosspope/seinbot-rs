extern crate glob;
extern crate markov;
extern crate rand;

use std::path::{PathBuf};
use markov::Chain;
use glob::glob;

use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;


fn main() {
    let actor_file = choose_actor();
    let actor = actor_file.display().to_string().replace("actors/", "").replace(".txt", "");
    
    print!("[{}] ", actor);
    for s in markov_gen(actor_file) {
        print!("{} ", capitalise_sentence(&s));
    }
    println!("")
}

fn capitalise_sentence(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}


fn choose_actor() -> PathBuf {
    let files: Vec<PathBuf> = glob("actors/*.txt").unwrap().filter_map(Result::ok).collect();
    let mut rng = thread_rng();
    let actor = files.choose(&mut rng).expect("the stage is not set");
    actor.to_owned()
}

fn markov_gen(script: PathBuf) -> Vec<String> {
    // Lower order: more random crazy stuff
    // Higher order: more likely to match original line (will need to strip text of punctuation - sentences on new lines)
    
    // TODO: Strip </font>
    // TODO: Strip opening/closing brackets that aren't a pair
    
    let mut chain = Chain::of_order(2);
    chain.feed_file(script).unwrap();
    
    let mut rng = thread_rng();
    chain.str_iter_for(rng.gen_range(1, 5)).map(|s| s.replace("</font>", "")).collect()
}
