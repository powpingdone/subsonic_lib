use super::*;
use const_format::{map_ascii_case, Case};
use deserialize::SubsonicResp;

macro_rules! api {
    ($name:tt) => {
        pub async fn $name(&self) -> anyhow::Result<SubsonicResp> {
            let base = self.make_url();
            const NAME: &str = map_ascii_case!(Case::Camel, "$name");
            Ok(self.make_req(base.0 + NAME + &base.1).await?)
        }
    };
}

impl SubsonicClient {
    api!(ping);
    api!(get_license);
    api!(get_music_folders);
}
