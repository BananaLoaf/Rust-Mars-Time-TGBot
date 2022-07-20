#[macro_use] extern crate log;
extern crate env_logger;

mod marstime;
mod bot;
mod handlers;

use tokio;
use marstime::MarsTime;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::thread;


fn main() {
    // Make use of logging level in env RUST_LOG=info
    env_logger::init();

    // Mutex
    // https://doc.rust-lang.org/std/sync/struct.Mutex.html
    // Arc
    // https://doc.rust-lang.org/std/sync/struct.Arc.html
    let mt = Arc::new(Mutex::new(MarsTime::new()));

    // Clone mt and pass it to update loop
    let mt_clone = mt.clone();
    thread::spawn(|| {MarsTime::update_loop(mt_clone)});

    // Clone mt and pass it to bot
    let mt_clone = mt.clone();
    let bot = bot::Bot::new(mt_clone);
    bot.init_handlers().start();
}
