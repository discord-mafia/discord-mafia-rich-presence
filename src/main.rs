use chrono;
use discord_rich_presence::{
    activity::{self, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use std::{thread, time::Duration};

fn main() {
    let mut client = match DiscordIpcClient::new("1143833637767348304") {
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
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        }

        loop {
            match client.set_activity(
                activity::Activity::new()
                    .state("discord.gg/social-deduction")
                    .details("Wanna play all things mafia?")
                    .timestamps(Timestamps::new().start(chrono::Utc::now().timestamp()))
                    .buttons(vec![Button::new(
                        "Join Discord",
                        "https://discord.gg/social-deduction",
                    )])
                    .assets(
                        Assets::new()
                            .large_image("discordmafia")
                            .large_text("Join Discord Mafia"),
                    ),
            ) {
                Ok(()) => println!("Set activity"),
                Err(err) => {
                    println!("Failed to set activity: {}", err);
                    break;
                }
            }

            thread::sleep(Duration::from_secs(15));
        }
    }
}
