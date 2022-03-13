use md5::{Digest, Md5};
use reqwest::{Client, ClientBuilder};
use thiserror::Error;

pub mod deserialize;
pub mod restapi;

use deserialize::SubsonicResp;

#[derive(Error, Debug)]
pub enum Error {
    #[error("the server is not new enough to use this api, server is {0}.{1}.{2} while api requested is {3}.{4}.{5}")]
    APIVersionMismatch(u32, u32, u32, u32, u32, u32),
}

#[derive(Debug)]
pub struct SubsonicClient {
    username: String,
    password: String,
    url: String,
    client: Client,
    ver_major: u32,
    ver_minor: u32,
    ver_bugfix: u32,
}

macro_rules! server_req {
    ($context:expr, $major:expr, $minor:expr, $bugfix:expr) => {{
        $context.ver_major >= $major
            && $context.ver_minor >= $minor
            && $context.ver_bugfix >= $bugfix
    }};
    ($context:expr, $major:expr, $minor:expr) => {{
        $context.ver_major >= $major && $context.ver_minor >= $minor
    }};
}

pub(crate) use server_req;

impl SubsonicClient {
    pub fn new(username: String, password: String, url: String) -> anyhow::Result<Self> {
        let raw: SubsonicResp = serde_xml_rs::from_str(
            reqwest::blocking::get(format!("{}/rest/ping", url))?
                .text()?
                .as_str(),
        )?;
        let version: Vec<&str> = raw.version.split(".").collect();
        let ver_major = version[0].parse::<u32>()?;
        let ver_minor = version[1].parse::<u32>()?;
        let ver_bugfix = version[2].parse::<u32>()?;
        let cl = ClientBuilder::new().build()?;

        Ok(SubsonicClient {
            username,
            password,
            url,
            ver_major,
            ver_minor,
            ver_bugfix,
            client: cl,
        })
    }

    fn salt_pass(&self) -> (String, String) {
        let mut md5 = Md5::new();
        // make salt from generating 10 random numbers and translating to hex
        let salt: String = std::iter::repeat_with(|| fastrand::u8(..=15))
            .take(10)
            .fold("".to_string(), |accum, x| accum + &format!("{:x?}", x));

        // hash the password and salt
        md5.update(self.password.clone() + &salt);
        let mut hash: [u8; 16] = Default::default();
        hash.copy_from_slice(&md5.finalize());

        // return the hash and salt
        (
            hash.iter()
                .copied()
                .fold("".to_string(), |accum, x| accum + &format!("{:x?}", x)),
            salt,
        )
    }

    async fn make_req(&self, url: String) -> anyhow::Result<SubsonicResp> {
        Ok(serde_xml_rs::from_str(
            &self.client.get(url).send().await?.text().await?,
        )?)
    }

    pub fn make_url(&self) -> (String, String) {
        (
            // base url
            self.url.clone() + "/rest/",
            // base url parameters
            format!(
                // username, version, format
                "?u={}&v={}.{}.{}&f=xml&",
                self.username, self.ver_major, self.ver_minor, self.ver_bugfix
            )
            // password
            + &(if server_req!(self, 1, 13) {
                // salted password
                let pair = self.salt_pass();
                format!("t={}&s={}", pair.0, pair.1)
            } else {
                // hex encoded password
                format!(
                    "p=enc:{}",
                    self.password
                        .clone()
                        .chars()
                        .fold("".to_string(), |accum, x| accum + &format!("{:x?}", x))
                )
            }),
        )
    }
}
