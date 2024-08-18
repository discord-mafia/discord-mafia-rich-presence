use chrono;
use discord_rich_presence::{
    activity::{self, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use tray_item::{IconSource, TIError, TrayItem};

fn presence(terminate: Arc<AtomicBool>) {
    let mut client = match DiscordIpcClient::new("1143833637767348304") {
        Ok(client) => client,
        Err(err) => {
            println!("Failed to create client: {}", err);
            return;
        }
    };

    let timestamp = Timestamps::new().start(chrono::Utc::now().timestamp());

    loop {
        if terminate.load(Ordering::SeqCst) {
            println!("Presence thread terminating.");
            break;
        }

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
            if terminate.load(Ordering::SeqCst) {
                println!("Presence thread terminating.");
                break;
            }

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

fn tray(terminate: Arc<AtomicBool>) -> Result<(), TIError> {
    let mut tray = TrayItem::new("Mafia Engine", IconSource::Resource(""))?;

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();

    let terminate_clone = Arc::clone(&terminate);
    tray.add_menu_item("Quit", move || {
        terminate_clone.store(true, Ordering::SeqCst);
        std::process::exit(0);
    })?;

    Ok(())
}

fn main() {
    let terminate = Arc::new(AtomicBool::new(false));

    let terminate_presence = Arc::clone(&terminate);
    let _ = thread::spawn(move || {
        presence(terminate_presence);
    });

    match tray(terminate) {
        Ok(_) => {
            println!("Tray icon created successfully");
        }
        Err(err) => {
            println!("Failed to create tray icon: {}", err);
            return;
        }
    }
}
