extern crate irc;

use irc::client::prelude::*;

#[no_mangle]
pub fn handle_message(client: &IrcClient, message: &Message) {
    if let Command::PRIVMSG(ref channel, ref msg) = message.command {
        if msg.contains("calzone") {
            client.send_privmsg(&channel, "Calzones are the best kind of sandwich.");
        }
    }
}

#[no_mangle]
pub fn initialize(_client: &IrcClient) {
    println!("Calzone arguer initialized.");
}

#[no_mangle]
pub fn finalize() {
    println!("Calzone arguer finalized.");
}

#[no_mangle]
pub fn print_description(client: &IrcClient, channel: &str) {
    client.send_privmsg(channel, "calzone: I tell people that calzones are sandwiches.");
}
