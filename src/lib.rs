use reqwest::{Client, ClientBuilder};
use fastrand::Rng;

mod serde_types;

#[derive(Debug)]
pub struct SubsonicClient {
    username: String,
    password: String,
    url: String,
    rng: Rng,
    ver_major: u32,
    ver_minor: u32,
    ver_bugfix: u32,
    client: Client,
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
    ($major:expr, $minor:expr, $bugfix:expr) => {
        {self.ver_major >= $major && self.ver_minor >= $minor && self.ver_bugfix >= $bugfix}
    }
}

impl SubsonicClient {
    pub fn new(username: String, password: String, url: String) -> Result<Self, Error> {
        let version: serde_types::SubsonicPing = serde_xml_rs::from_str(reqwest::blocking::get(format!("{}/rest/ping", url))?.text()?.as_str())?;
        let version: Vec<&str> = version.version.split(".").collect();
        if version.len() != 3 {
            return Err(Error::ParseErr);
        }
        let ver_major = version[0].parse::<u32>()?;
        let ver_minor = version[1].parse::<u32>()?;
        let ver_bugfix = version[2].parse::<u32>()?;

        let cl = ClientBuilder::new().build()?;
        Ok(
            SubsonicClient {
                username,
                password,
                url,
                ver_major,
                ver_minor,
                ver_bugfix,
                rng: Rng::new(),
                client: cl,
            }
        )
    }

    pub fn make_url(&self) -> String {
        "".to_string()
    }
}

