use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("843514276383031296")?;

    client.connect()?;
    client.set_activity(activity::Activity::new().state("foo").details("bar"))?;

    loop {
        println!("foo");
    }
}
