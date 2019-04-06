mod config;
mod text_generation;
mod twitter;

use crate::text_generation::actor::{choose_actor, generate_dialogue};
use crate::text_generation::elaine::ELAINE;
use crate::text_generation::frank::FRANK;
use crate::text_generation::george::GEORGE;
use crate::text_generation::jerry::JERRY;
use crate::text_generation::kramer::KRAMER;
use crate::text_generation::newman::NEWMAN;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, debug, error, info};
use serde_derive::Deserialize;
use simple_error::bail;
use simple_logger;
use std::error::Error;

#[derive(Deserialize)]
struct SeinbotEvent {}

const TWEET_CHARACTER_LIMIT: usize = 280;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(seinbot);
    Ok(())
}

fn seinbot(_: SeinbotEvent, _: Context) -> Result<(), HandlerError> {
    let conf = match config::Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            error!("{}", e);
            bail!("failed to load seinbot environment variables");
        }
    };

    // Get the tweet history of the bot for analysis
    let mut bot = twitter::TwitterBot::new(conf);
    let history = bot.history(200);

    // Choose the next actor to tweet, ignoring the last 4 who posted
    let actors_to_ignore = last_actors(&history, 4);
    let actors = vec![JERRY, ELAINE, FRANK, GEORGE, KRAMER, NEWMAN];
    let actor = choose_actor(&actors, &actors_to_ignore);
    info!("{} is tweeting", actor.name);

    let mut tweet: String;
    loop {
        let statement = generate_dialogue(&actor, 2, 3);
        tweet = format!("[{}] {}", actor.name, statement);

        // Make sure tweet is within post limit
        if tweet.len() > TWEET_CHARACTER_LIMIT {
            debug!("too large: {}", tweet);
            continue;
        }

        // Check that the statement has not already been said
        if history.iter().any(|t| t == &tweet) {
            debug!("already tweeted: {}", tweet);
            continue;
        }

        // We've passed all the checks
        break;
    }

    info!("{}", tweet);
    bot.tweet(&tweet);
    Ok(())
}

/// Based on previous tweet history, find the last actor who tweeted
fn last_actors(tweets: &[String], n: usize) -> Vec<String> {
    let mut last_actors = Vec::new();
    for tweet in tweets.iter().take(n) {
        let actor = tweet
            .split(' ')
            .collect::<Vec<&str>>()
            .first()
            .expect("tweet is not in expected format")
            .replace("[", "")
            .replace("]", "");
        
        last_actors.push(actor);
    }
    
    info!("last {} actors to post: {:?}", n, last_actors);
    
    last_actors
}
