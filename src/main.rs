#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate futures;
extern crate glob;
extern crate markov;
extern crate rand;
extern crate serde_json;
extern crate tokio;
extern crate tokio_core;

use glob::glob;
use markov::Chain;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::path::{Path, PathBuf};
use tokio_core::reactor::Core;

mod config;
mod twitter;

fn main() {
    let mut core = Core::new().unwrap();
    let conf = config::Config::read(Path::new("twitter-secrets.json"));
    let bot = twitter::TwitterBot::new(&mut core, conf.unwrap());

    let script_file = choose_actor();
    let actor = script_file
        .display()
        .to_string()
        .replace("actors/", "")
        .replace(".txt", "");

    let line = generate_lines(script_file).join(" ");
    let formatted = format!("[{}] {}", actor, line);

    println!("{}", formatted);
}

fn choose_actor() -> PathBuf {
    let files: Vec<PathBuf> = glob("actors/*.txt")
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    let mut rng = thread_rng();
    let actor = files.choose(&mut rng).expect("the stage is not set");
    actor.to_owned()
}

fn generate_lines(script: PathBuf) -> Vec<String> {
    // Order of 2 seems to be the sweet spot in generating crazy text
    let mut chain = Chain::of_order(2);
    chain.feed_file(script).unwrap();

    // Generate a random set of of sentences (between 1 and 5)
    // TODO:
    //  * Strip opening/closing brackets that aren't a pair
    //  * Check that generated line is within tweet character limit
    //  * Generated phrase is disimalar to line already in script + posted to twitter
    let mut rng = thread_rng();
    chain
        .str_iter_for(rng.gen_range(2, 4))
        .map(|s| capitalise_sentence(&s))
        .collect()
}

fn capitalise_sentence(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
