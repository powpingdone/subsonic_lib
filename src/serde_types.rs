use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubsonicPing {
    pub status: String,
    pub version: String,
}
