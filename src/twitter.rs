// use self::tokio::runtime::current_thread::block_on_all;
use egg_mode;
use tokio::runtime::current_thread::block_on_all;
use tokio_core::reactor::Core;

use crate::config::Config;

pub struct TwitterBot {
    pub token: egg_mode::Token,
}

impl TwitterBot {
    pub fn new(core: &mut Core, config: Config) -> Self {
        let handle = core.handle();
        let consumer_token = egg_mode::KeyPair::new(config.consumer_key, config.consumer_secret);
        let access_token = egg_mode::KeyPair::new(config.access_key, config.access_secret);

        let token = egg_mode::Token::Access {
            consumer: consumer_token,
            access: access_token,
        };

        match core.run(egg_mode::verify_tokens(&token, &handle)) {
            Err(err) => panic!("Unable to verify tokens: {:?}", err),
            Ok(user) => println!("{} : {}", user.id, user.screen_name),
        }

        TwitterBot { token }
    }

    // pub fn tweet (&self, message : String)  {
    //     twitter_api::update_status(&self.consumer_token, &self.access_token, &message).unwrap();
    // }
}
