extern crate irc;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use irc::client::prelude::*;
use regex::Regex;

lazy_static! {
    static ref JOIN_REGEX: Regex = Regex::new(r"!join (.*) ?(.*)?").unwrap();
    static ref PART_REGEX: Regex = Regex::new(r"!part (.*)").unwrap();
}

#[no_mangle]
pub fn handle_message(client: &IrcClient, message: &Message) {
    if let Command::PRIVMSG(ref source_chan, ref msg) = message.command {
        let nick = match message.source_nickname() {
            Some(nick) => nick,
            None => "",
        };
        if !client.config().is_owner(nick) {
            return;
        }
        // match join commands
        if let Some(caps) = JOIN_REGEX.captures(msg) {
            if let Some(channel) = caps.get(1) {
                if let Some(chankey) = caps.get(2) {
                    match client.send_join_with_keys(&channel.as_str(), &chankey.as_str()) {
                        Ok(_) => {},
                        Err(_) => {
                            client.send_privmsg(&source_chan, "Unable to join channel.").unwrap();
                        }
                    };
                } else {
                    match client.send_join(&channel.as_str()) {
                        Ok(_) => {},
                        Err(_) => {
                            client.send_privmsg(&source_chan, "Unable to join channel.").unwrap();
                        }
                    };
                }
            }
        }
        // match part command
        if let Some(caps) = PART_REGEX.captures(msg) {
            if let Some(channel) = caps.get(1) {
                match client.send_part(&channel.as_str()) {
                    Ok(_) => {},
                    Err(_) => {
                        client.send_privmsg(&source_chan, "Unable to part channel.").unwrap();
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub fn initialize(_client: &IrcClient) {
    println!("Join/Part plugin initialized.");
}

#[no_mangle]
pub fn finalize() {
    println!("Join/Part plugin finalized.");
}


#[no_mangle]
pub fn print_description(client: &IrcClient, channel: &str) {
    client.send_privmsg(channel, "joinpart: Handles joining and leaving channels at runtime.");
}
