extern crate ammonia;
#[macro_use]
extern crate log;
extern crate number_prefix;
extern crate pretty_env_logger;
extern crate reqwest;
extern crate retry;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_xml_rs;

mod goodreads;
mod telegram;

pub use telegram::TelegramBot;
