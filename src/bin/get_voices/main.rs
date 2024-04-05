use playht_rs::{
    api::{self, voice::get_voices},
    prelude::*,
};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let voices = get_voices().await?;

    println!("Got {} voices", voices.len());

    let c = api::Client::new();

    let voices = c.get_voices().await?;

    println!("Got {} voices", voices.len());

    Ok(())
}
