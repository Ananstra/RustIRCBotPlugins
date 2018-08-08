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
