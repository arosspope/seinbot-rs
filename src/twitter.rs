// use self::tokio::runtime::current_thread::block_on_all;
use egg_mode;
use tokio_core::reactor::Core;
use log::{info, error};

use crate::config::Config;

pub struct TwitterBot {
    pub token: egg_mode::Token,
    pub user_id: u64,
    core: Core,
}

impl TwitterBot {
    pub fn new(config: Config) -> Self {
        let mut core = Core::new().expect("unable to create tokio core");
        let handle = core.handle();
        let consumer_token = egg_mode::KeyPair::new(config.consumer_key, config.consumer_secret);
        let access_token = egg_mode::KeyPair::new(config.access_key, config.access_secret);

        let token = egg_mode::Token::Access {
            consumer: consumer_token,
            access: access_token,
        };

        match core.run(egg_mode::verify_tokens(&token, &handle)) {
            Err(err) => panic!("Unable to verify tokens: {:?}", err),
            Ok(user) => TwitterBot { token: token , user_id: user.id, core: core },
        }
    }

    pub fn tweet(&mut self, message: &str)  {
        let handle = self.core.handle();
        match self.core.run(egg_mode::tweet::DraftTweet::new(message).send(&self.token, &handle)) {
            Err(err) => error!("failed to post: {}", err),
            Ok(_) => info!("successfully posted to twitter"),
        }
        
    }
    
    pub fn history(&mut self, max_records: usize) -> Vec<String> {
        let handle = self.core.handle();
        let home = egg_mode::tweet::user_timeline(self.user_id, false, false, &self.token, &handle).with_page_size(max_records as i32);
        let (_, feed) = self.core.run(home.start()).expect("unable to load bot feed");
        
        feed.iter().map(|tweet| tweet.text.to_owned()).collect()
    }
}
