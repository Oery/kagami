use dotenvy::dotenv;

mod kagami;
pub use kagami::Kagami;

pub mod minecraft;
pub mod serialization;
pub mod tcp;

use minecraft::packets::play;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let remote_addr = std::env::var("SERVER_ADDR")
        .expect("There must be an .env file with SERVER_ADDR set to whatever server you want to connect to")
        .parse()?;

    let mut proxy = Kagami::new(remote_addr);

    proxy.register_callback(|packet: &play::server::Login| {
        dbg!(packet);

        kagami::callbacks::Actions::Filter
    });

    proxy.run().await?;

    Ok(())
}

// TODO: Notification when packet handler crashes
// TODO: Kagami CLI to quickly create a proxy debugger
