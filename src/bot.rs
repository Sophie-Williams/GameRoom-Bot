use discord;
use discord::{Discord, Connection};
use discord::model::{Event, ReactionEmoji, Message};

use game::Game;
use command::Command;

use BOT_TOKEN;

//#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBot {
    discord: Discord,
    connection: Connection,
    library: Vec<Game>,
}

impl GameBot {
    pub fn new() -> GameBot {
        let discord = Discord::from_bot_token(&*BOT_TOKEN).expect("Login failed");
        let (mut connection, _) = discord.connect().expect("Connection failed");
        GameBot{
            discord: discord,
            connection: connection,
            library: Vec::new(),
        }
    }
    
    pub fn main(&mut self) {
        println!("Ready");
        let mut exit = false;
        while !exit {
            match self.connection.recv_event() {
                Ok(Event::MessageCreate(message)) => {
                    //let _ = self.discord.add_reaction(message.channel_id, message.id, ReactionEmoji::Unicode("🤔".to_string()));
                    /*match &*message.content {
                        "!test" => {
                            let _ = discord.send_message(message.channel_id, "This is a reply to the test.", "", false);
                        }
                        "!quit" => {
                            println!("{} called quit", message.author.name);
                            exit = true;
                        }
                        _ => {}
                    }*/
                    if message.content.chars().next() == Some('~') {
                        self.command_handler(&message);
                    }
                }
                Ok(_) => {}
                Err(discord::Error::Closed(code, body)) => {
                    println!("Gateway closed on us with code {:?}: {}", code, body);
                    exit = true;
                }
                Err(err) => println!("Receive error: {:?}", err)
            }
        }
        //self.write_library(LIB_FILE);
        println!("Exited");
    }
    
    fn command_handler(&mut self, message: &Message) {
        //println!("{:?}", message);
        let com = Command::parse(message);
        //println!("{:?}", com);
        match com.command() {
            "~add" => self.add_game(&com),
            //"~echo" => self.echo(&com),
            //"~exit" => self.exit(&com),
            _ => self.com_error(&com, 1)
        }
    }
    
    fn add_game(&mut self, com: &Command) {
        if com.args().len() == 2 {
            match com.args()[1].parse::<i32>() {
                Ok(limit) => {
                    let name = com.args()[0].clone().to_uppercase();
                    let new_game = Game::new(name, limit);
                    self.library.push(new_game.clone());
                    
                    let new_game_str = format!("Added {} with limit {} to library of games", new_game.name(), new_game.limit());
                    self.com_msg(com, new_game_str);
                },
                Err(_) => self.com_error(com, 3)
            }
        } else {
            self.com_error(com, 2);
        }
    }
    
    fn com_msg(&self, com: &Command, msg: String) {
        let _ = self.discord.send_message(*com.channel_id(), &msg, "", false);
    }
    
    fn com_error(&self, com: &Command, errno: u32) {
        let mut err_str: String;// = String::new();
        match errno {
            1 => err_str = format!("Error: Unknown command \"{}\".", com.command()),
            2 => err_str = format!("Error: Invalid number of arguments for command \"{}\".", com.command()),
            3 => err_str = format!("Error: Invalid argument for command \"{}\".", com.command()),
            _ => err_str = format!("Error: An unknown error occurred.")
        }
        self.com_msg(com, err_str);
    }
}
