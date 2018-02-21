#[macro_use]
extern crate text_io;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate discord;

mod game;
mod command;
mod bot;

use bot::GameBot;
use std::string::String;

//const BOT_TOKEN_FILE: &str = "../res/bot_token";

const BOT_TOKEN: &'static str = include_str!("../res/bot_token");
//const LIB_FILE: &str = "./lib_file";
const LIB_FILE: &str = "/home/ubuntu/Desktop/lib_file";

fn main() {
    //println!("{}", &*BOT_TOKEN);
    let mut game_bot = GameBot::new();
    game_bot.main();
}
