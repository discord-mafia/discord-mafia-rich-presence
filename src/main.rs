use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::{thread, time::Duration};

fn main() {
    let mut client = match DiscordIpcClient::new("843514276383031296") {
        Ok(client) => client,
        Err(err) => {
            println!("Failed to create client: {}", err);
            return;
        }
    };

    loop {
        match client.connect() {
            Ok(()) => {
                println!("Connected to Discord");
            }
            Err(err) => {
                println!("Failed to connect: {}", err);
                thread::sleep(Duration::from_secs(5)); // wait 5 seconds before retrying
                continue;
            }
        }

        loop {
            match client.set_activity(activity::Activity::new().state("foo").details("bar")) {
                Ok(()) => println!("Set activity"),
                Err(err) => {
                    println!("Failed to set activity: {}", err);
                    break; // if setting activity fails, break out of the loop and try to reconnect
                }
            }

            thread::sleep(Duration::from_secs(15)); // update activity every 15 seconds
        }
    }
}
