use std::env;

use revolt_rs::{http::client::Client, gateway::WebSocketClient};

#[tokio::main]
async fn main() -> anyhow::Result<()>   {
    tracing_subscriber::fmt::init(); 

    let client = Client::builder()
    .token(env::var("REVOLT_TOKEN")?)
    .is_bot(Some(false))
    .exec(); 

    let user = client.fetch_user("01H2VSNEK6A2F5JAMB5GE6R94E").await?; 

    println!("Hello, {:?}!", user.username);
    
    Ok(())
}
