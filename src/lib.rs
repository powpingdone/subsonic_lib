use md5::{Digest, Md5};
use reqwest::{Client, ClientBuilder};

mod serde_types;

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

#[derive(Debug)]
pub enum Error {
    SerdeErr(serde_xml_rs::Error),
    RequestErr(reqwest::Error),
    ParseErr,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestErr(err)
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(err: serde_xml_rs::Error) -> Self {
        Self::SerdeErr(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::ParseErr
    }
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

impl SubsonicClient {
    pub fn new(username: String, password: String, url: String) -> Result<Self, Error> {
        let version: serde_types::SubsonicPing = serde_xml_rs::from_str(
            reqwest::blocking::get(format!("{}/rest/ping", url))?
                .text()?
                .as_str(),
        )?;
        let version: Vec<&str> = version.version.split(".").collect();
        if version.len() != 3 {
            return Err(Error::ParseErr);
        }
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
        // make salt from generating 6 random numbers and translating to hex
        let salt: String = std::iter::repeat_with(|| fastrand::u8(..=15))
            .take(6)
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

    pub fn make_url(&self) -> (String, String) {
        (
            self.url.clone() + "/rest/",
            format!(
                "?u={}&v={}.{}.{}&f=xml&",
                self.username, self.ver_major, self.ver_minor, self.ver_bugfix
            ) + &(if server_req!(self, 1, 13) {
                let pair = self.salt_pass();
                format!("t={}&s={}", pair.0, pair.1)
            } else {
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
