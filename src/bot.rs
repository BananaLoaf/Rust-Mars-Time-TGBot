use std::{env, net::SocketAddr};
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;
use carapax::{Api, Config, Dispatcher};
use carapax::{webhook::run_server, longpoll::LongPoll};

use super::marstime::MarsTime;
use super::handlers;


pub struct Bot {
    api: Api,
    dispatcher: Dispatcher<BotContext>,
}

pub struct BotContext {
    pub api: Api,
    pub mt: Arc<Mutex<MarsTime>>
}


impl Bot {
    pub fn new(mt: Arc<Mutex<MarsTime>>) -> Bot {
        let token: String = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN is not set");
        let config: Config = match env::var("TELEGRAM_PROXY") {
            Ok(proxy) => Config::new(token).proxy(proxy).unwrap(),
            Err(_e) => {
                warn!("TELEGRAM_PROXY is not set!");
                Config::new(token)
            },
        };

        let api = Api::new(config).unwrap();
        let dispatcher = Dispatcher::new(BotContext {api: api.clone(), mt: mt});

        return Bot {
            api,
            dispatcher,
        }
    }

    pub fn init_handlers(mut self) -> Self {
        self.dispatcher.add_handler(handlers::start_handler);
        self.dispatcher.add_handler(handlers::command_handler);

        return self
    }

    #[tokio::main]
    pub async fn start(self) {
        match env::var("TELEGRAM_ADDRESS")  // TELEGRAM_ADDRESS=127.0.0.1:8080
        {
            Ok(address) => {
                let address: SocketAddr = address.parse().expect("Address is invalid");
                info!("Starting server at {}", address);
                run_server(address, "/", self.dispatcher).await.unwrap_or_else(|_e| {
                    error!("Error starting server at {}\n{}", address, _e);
                });
            },

            Err(_e) => {
                warn!("TELEGRAM_ADDRESS for webhook is not set");
                warn!("Use format 127.0.0.1:8080");
                info!("Long polling");
                LongPoll::new(self.api, self.dispatcher).run().await;
            }
        };
    }
}