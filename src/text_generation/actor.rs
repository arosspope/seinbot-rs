use markov::Chain;
use rand::{seq::SliceRandom, thread_rng, Rng};

pub struct Actor {
    pub name: &'static str,
    pub lines: &'static [&'static str],
    pub markov_order: usize,
}

pub fn choose_actor<'a>(actors: &'a [Actor], actors_to_ignore: &[String]) -> &'a Actor {
    let mut rng = thread_rng();
    actors
        .into_iter()
        .filter(|a| !actors_to_ignore.iter().any(|i| a.name == i))
        .collect::<Vec<&Actor>>()
        .choose(&mut rng)
        .expect("choosing actor failed")
}

pub fn generate_dialogue(actor: &Actor, min_statements: usize, max_statements: usize) -> String {
    let mut rng = thread_rng();
    let mut chain = Chain::of_order(actor.markov_order);

    for line in actor.lines {
        chain.feed_str(line);
    }

    chain
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
