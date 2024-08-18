use chrono;
use discord_rich_presence::{
    activity::{self, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use std::{thread, time::Duration};

fn presence() {
    let mut client = match DiscordIpcClient::new("1143833637767348304") {
        Ok(client) => client,
        Err(err) => {
            println!("Failed to create client: {}", err);
            return;
        }
    };

    let timestamp = Timestamps::new().start(chrono::Utc::now().timestamp());

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
                    .details("Wanna play all things mafia?")
                    .timestamps(timestamp.clone())
                    .buttons(vec![Button::new(
                        "Join Discord Mafia",
                        "https://discord.gg/social-deduction",
                    )])
                    .assets(
                        Assets::new()
                            .large_image("discordmafia")
                            .large_text("Join Discord Mafia")
                            .small_image("cog_icon")
                            .small_text("We have our own bot!"),
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

fn main() {
    presence();
}
