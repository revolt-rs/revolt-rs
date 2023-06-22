use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EmbedProvider {
    Gif,
    YouTube,
    Lightspeed,
    Twitch,
    Spotify,
    Soundcloud,
    Bandcamp,
    Streamable,
    None,
}
