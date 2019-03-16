extern crate glob;
extern crate markov;
extern crate rand;

use std::path::{PathBuf};
use markov::Chain;
use glob::glob;

use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;


fn main() {
    let script_file = choose_actor();
    let actor = script_file.display().to_string().replace("actors/", "").replace(".txt", "");
    
    let line = generate_lines(script_file).join(" ");
    let formatted = format!("[{}] {}", actor, line);
    
    println!("{}", formatted);
}

fn choose_actor() -> PathBuf {
    let files: Vec<PathBuf> = glob("actors/*.txt").unwrap().filter_map(Result::ok).collect();
    let mut rng = thread_rng();
    let actor = files.choose(&mut rng).expect("the stage is not set");
    actor.to_owned()
}

fn generate_lines(script: PathBuf) -> Vec<String> {
    // Lower order: more random crazy stuff
    // Higher order: more likely to match original line (will need to strip text of punctuation - sentences on new lines)
    // TODO: Strip opening/closing brackets that aren't a pair
    
    let mut chain = Chain::of_order(2);
    chain.feed_file(script).unwrap();
    
    // Generate a random set of of sentences (between 1 and 5)
    let mut rng = thread_rng();
    chain.str_iter_for(rng.gen_range(1, 5)).map(|s| capitalise_sentence(&s)).collect()
}

fn capitalise_sentence(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}