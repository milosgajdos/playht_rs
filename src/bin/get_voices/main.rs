use playht_rs::{api::voice::get_voices, prelude::*};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let voices = get_voices().await?;

    println!("Got voices: {:?}", voices);

    Ok(())
}
