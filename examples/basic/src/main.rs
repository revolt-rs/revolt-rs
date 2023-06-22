use std::env;

use revolt_rs::{
    http::client::Client,
    model::{channel::message::embed::EmbedBuilder, id::Id},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = Client::builder()
        .token(env::var("TOKEN")?)
        .is_bot(false)
        .exec();

    let channel_id = Id::new("01H2Y55Y9BMGNQQJTKQJ3SVKTV");

    let embed = EmbedBuilder::new()
        .title("Hello world")
        .build();

    client
        .create_message(channel_id)
        .await
        .content("Hello world")?
        .embeds(embed)?
        .send()
        .await;

    let me = client.current_user().await?;

    println!("{:?}", me.tag());

    Ok(())
}
