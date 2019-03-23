use lambda_runtime::{error::HandlerError, lambda, Context};
use getopts::Options;
use glob::glob;
use log::{self, info, error};
use markov::Chain;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::{env, fs};
use std::path::{Path, PathBuf};
use simple_error::{try_with, bail};
use simple_logger;
use std::error::Error;
use serde_derive::{Deserialize};

mod config;
mod twitter;

const TWEET_CHARACTER_LIMIT: usize = 280;

#[derive(Deserialize)]
struct SeinbotPostEvent {}


fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?; 
    lambda!(seinbot_lambda);
    Ok(())   
}


fn seinbot_lambda(_: SeinbotPostEvent, _: Context) -> Result<(), HandlerError> {
    let conf = match config::Config::from_env() {
        Ok(c) => c,
        Err(_) => bail!("failed to load file"),
    };
    
    // let current_dir = env::current_dir().unwrap();
    
    // Get the tweet history of the bot for analysis
    let mut bot = twitter::TwitterBot::new(conf);
    let history = bot.history(200);
    
    // Choose the next actor to tweet, ignoring the last one who posted
    let script = match choose_script("actors", last_actor(&history)) {
        Ok(s) => s,
        Err(_) => bail!("failed to load script"),
    };
    
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

    // Post to twitter!
    info!("posting: {}", tweet);
    bot.tweet(&tweet);
    Ok(())
}

/// Randomly choose a script from the scripts folder
fn choose_script(script_location: &str, ignore_actor: Option<String>) -> Result<PathBuf, Box<Error>> {
    let mut scripts: Vec<PathBuf> = glob(&format!("{}/*.txt", script_location))
        .unwrap()
        .filter_map(Result::ok)
        .collect();
        
    info!("{:?}", scripts);
    
    if let Some(ignore) = ignore_actor.clone() {
        scripts = scripts
            .into_iter()
            .filter(|s| s != &PathBuf::from(format!("{}/{}.txt", script_location, ignore)))
            .collect();
    }
    
    
    
    info!(
        "choosing actors and ignoring {}",
        ignore_actor.unwrap_or(String::from("nobody"))
    );
    
    let mut rng = thread_rng();
    let script = scripts.choose(&mut rng).expect("the stage is not set");
    Ok(script.to_owned())
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
            .first()
            .expect("tweet is not in expected format")
            .replace("[", "")
            .replace("]", "");

        return Some(actor);
    }

    None
}
