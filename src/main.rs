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
use log::{info};
use markov::Chain;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::path::{Path, PathBuf};
use getopts::Options;
use std::env;

mod config;
mod twitter;

const TWEET_CHARACTER_LIMIT: usize = 280;

fn main() {
    // Parse command line options
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("p", "post", "will post the generated message to twitter");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    // Load configuration and create handle to twitter bot
    let conf = config::Config::read(Path::new("twitter-secrets.json")).expect("loading secrets failed");
    let mut bot = twitter::TwitterBot::new(conf);
    
    // Get the tweet history of the bot for analysis
    let history = bot.history(200);
    
    // Choose the next actor to tweet, ignoring the last one who posted
    let script = choose_script("actors", last_actor(&history));
    let actor = script
        .display()
        .to_string()
        .replace("actors/", "")
        .replace(".txt", "");
    info!("choose {} to tweet", actor);
    
    // Now the fun part! Generate markov text!
    let mut tweet: String;
    loop {
        let statement = generate_content(&script).join(" ");
        tweet = format!("[{}] {}", actor, statement);
        
        // Make sure tweet is within post limit
        if tweet.len() > TWEET_CHARACTER_LIMIT {
            continue;
        }
        
        // Check that the statement has not already been said
        if history.iter().any(|t| t == &tweet) {
            continue;
        }
        
        // We've passed all the checks
        break;
    }

    // Print to terminal and post to twitter!
    println!("{}", tweet);
    if matches.opt_present("p") {
        info!("posting to twitter");
        bot.tweet(&tweet);
    }
}

/// Randomly choose a script from the scripts folder
fn choose_script(script_location: &str, ignore_actor: Option<String>) -> PathBuf {
    let mut scripts: Vec<PathBuf> = glob(&format!("{}/*.txt", script_location))
        .unwrap()
        .filter_map(Result::ok)
        .collect();
        
    if let Some(ignore) = ignore_actor.clone() {
        scripts = scripts
            .into_iter()
            .filter(|s| s != &PathBuf::from(format!("{}/{}.txt", script_location, ignore)))
            .collect();
    }
    
    info!("choosing actors and ignoring {}", ignore_actor.unwrap_or(String::from("nobody")));
    let mut rng = thread_rng();
    let script = scripts.choose(&mut rng).expect("the stage is not set");
    script.to_owned()
}

/// Generate content based on the input script file
fn generate_content(script: &PathBuf) -> Vec<String> {
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

/// Capitalise a sentence
fn capitalise_sentence(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Based on previous tweet history, find the last actor who tweeted
fn last_actor(tweets: &[String]) -> Option<String> {
    if tweets.len() > 0 {
        let actor = tweets[0]
            .split(' ')
            .collect::<Vec<&str>>()
            .first().expect("tweet is not in expected format")
            .replace("[", "")
            .replace("]", "");

        return Some(actor);
    }
    
    None
}
