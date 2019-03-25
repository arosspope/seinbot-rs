// Actor imports
// mod elaine;
// mod frank;
// mod george;
// mod jerry;
// mod kramer;
// mod newman;
use markov::Chain;
use rand::{thread_rng, Rng};

use super::jerry::JERRY;

pub struct Actor {
    pub name: String,
    lines: Vec<String>,
    markov_order: usize,
}

pub fn choose_actor(ignore: &str) -> &Actor {
    let mut rng = thread_rng();
    let actors = vec![JERRY];
    actors.into_iter()
        .filter(|&a| a.name != ignore)
        .collect::<Vec<Actor>>()
        .choose(&mut rng)
        .expect("choosing actor failed")
}

pub fn generate_dialogue(actor: &Actor, min_statements: usize, max_statements: usize) -> String {
    let mut rng = thread_rng();
    let mut chain = Chain::of_order(actor.markov_order);
    chain.feed(actor.lines)
        .str_iter_for(rng.gen_range(min_statements, max_statements))
        .map(|s| capitalise_sentence(&s))
        .collect::<Vec<String>>()
        .join(" ")
}

fn capitalise_sentence(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}