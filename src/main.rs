#[macro_use]
extern crate text_io;

#[macro_use]
extern crate lazy_static;

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

lazy_static! {
    static ref BOT_TOKEN: String = read_bot_token();
}

fn main() {
    //println!("{}", &*BOT_TOKEN);
    let mut game_bot = GameBot::new();
    game_bot.main();
}

fn read_bot_token() -> String {
    let bot_token_u8 = include_bytes!("../res/bot_token");
    let mut token_string = String::new();
    for c in bot_token_u8.iter() {
        token_string.push((*c) as char);
    }
    token_string
}
