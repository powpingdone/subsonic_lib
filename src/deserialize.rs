use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsonicResp {
    pub status: String,
    pub version: String,
    #[serde(rename = "$value")]
    pub resp: Option<SubsonicInfo>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SubsonicInfo {
    #[serde(alias = "newestPodcasts")]
    #[serde(alias = "nowPlaying")]
    #[serde(alias = "randomSongs")]
    #[serde(alias = "similarSongs")]
    #[serde(alias = "similarSongs2")]
    #[serde(alias = "songsByGenre")]
    #[serde(alias = "topSongs")]
    #[serde(alias = "videos")]
    MediaList(GenericList<Media>),

    Album(Album),
    #[serde(alias = "albumList2")]
    AlbumList(GenericList<Album>),
    Artist(Artist),
    Artists(Artists),
    Bookmarks(GenericList<Bookmark>),
    ChatMessages(GenericList<ChatMessage>),
    Directory(Directory),
    Genres(GenericList<Genre>),
    Indexes(Indexes),
    InternetRadioStations(GenericList<Radio>),
    JukeboxPlaylist(JukeboxPlaylist),
    MusicFolders(GenericList<MusicFolder>),
    Playlist(Playlist),
    Playlists(GenericList<Playlist>),
    PlayQueue(PlayQueue),
    Podcasts(GenericList<Channel>),
    SearchResult(SearchResult),
    #[serde(alias = "searchResult2")]
    #[serde(alias = "searchResult3")]
    #[serde(alias = "starred")]
    #[serde(alias = "starred2")]
    SearchOrStarList(GenericList<SubsonicInfo>),
    Shares(GenericList<Share>),
    Song(Media),
    User(User),
    Users(GenericList<User>),
    VideoInfo(VideoInfo),

    #[serde(rename_all = "camelCase")]
    Error {
        code: u32,
        message: String,
    },
    #[serde(rename_all = "camelCase")]
    JukeboxStatus {
        current_index: u64,
        playing: bool,
        gain: f32,
        position: u64,
    },
    #[serde(rename_all = "camelCase")]
    License {
        valid: bool,
        email: String,
        license_expires: String,
    },
    #[serde(rename_all = "camelCase")]
    Lyrics {
        artist: String,
        title: String,
        #[serde(rename = "$value")]
        lyrics: String,
    },
    #[serde(alias = "albumInfo")]
    #[serde(alias = "artistInfo2")]
    #[serde(alias = "artistInfo")]
    #[serde(rename_all = "camelCase")]
    PublicInfo {
        biography: Option<String>,
        notes: Option<String>,
        music_brainz_id: Option<String>,
        last_fm_url: Option<String>,
        small_image_url: Option<String>,
        medium_image_url: Option<String>,
        large_image_url: Option<String>,
        #[serde(rename = "similarArtist")]
        similar_artists: Option<Vec<Artist>>,
    },
    #[serde(rename_all = "camelCase")]
    ScanStatus {
        scanning: bool,
        count: Option<u32>,
    },
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Entrires {
    AllowedUser(String),
    Entry(Media),
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IndexChoice {
    #[serde(rename_all = "camelCase")]
    Shortcut {
        id: u64,
        name: String,
    },
    Index(Index),
    Child(Media),
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VideoProperties {
    #[serde(rename_all = "camelCase")]
    AudioTrack {
        id: u64,
        name: String,
        language_code: String,
    },
    #[serde(rename_all = "camelCase")]
    Captions { id: u64, name: String },
    #[serde(rename_all = "camelCase")]
    Conversion { id: u64, bit_rate: u32 },
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: u64,
    #[serde(alias = "name")]
    pub title: String,

    pub artist: Option<String>,
    pub artist_id: Option<u64>,
    pub average_rating: Option<String>,
    pub cover_art: Option<String>,
    pub created: Option<String>,
    pub duration: Option<u64>,
    pub parent: Option<u64>,
    pub song_count: Option<u32>,
    pub user_rating: Option<String>,
    #[serde(rename = "$value")]
    pub songs: Option<Vec<Media>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub album_count: Option<u32>,
    pub cover_art: Option<String>,
    pub starred: Option<String>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artists {
    pub ignored_articles: Option<String>,
    #[serde(rename = "$value")]
    pub indexes: Option<Vec<Index>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    pub position: u64,
    pub username: String,
    pub comment: Option<String>,
    pub created: Option<String>,
    pub changed: Option<String>,
    #[serde(rename = "$value")]
    pub entries: Option<Vec<Media>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: u64,
    pub status: String,
    pub url: String,

    pub cover_art: Option<String>,
    pub description: Option<String>,
    pub error_message: Option<String>,
    pub original_image_url: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "$value")]
    pub episodes: Option<Vec<Media>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub username: String,
    pub time: u64,
    pub message: String,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    id: u64,
    parent: u64,
    name: String,
    starred: Option<String>,
    #[serde(rename = "$value")]
    children: Option<Vec<Media>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericList<T> {
    #[serde(rename = "$value")]
    pub items: Option<Vec<T>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub song_count: u32,
    pub album_count: u32,
    #[serde(rename = "$value")]
    pub name: String,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Indexes {
    pub last_modified: u64,
    pub ignored_articles: Option<String>,
    #[serde(rename = "$value")]
    pub list: Option<Vec<IndexChoice>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub name: String,
    #[serde(rename = "$value")]
    pub artists: Option<Vec<Artist>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JukeboxPlaylist {
    pub current_index: u64,
    pub playing: bool,
    pub gain: f32,
    pub position: u64,
    #[serde(rename = "$value")]
    pub list: Option<Vec<Media>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: u64,
    pub is_dir: bool,
    pub parent: u64,
    #[serde(alias = "name")]
    pub title: String,

    pub album: Option<String>,
    pub album_id: Option<u64>,
    pub artist: Option<String>,
    pub artist_id: Option<u64>,
    pub bit_rate: Option<u32>,
    pub channel_id: Option<u32>,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub cover_art: Option<String>,
    pub created: Option<String>,
    pub duration: Option<u32>,
    pub genre: Option<String>,
    pub is_video: Option<bool>,
    pub minutes_ago: Option<u32>,
    pub path: Option<String>,
    pub player_id: Option<u32>,
    pub player_name: Option<String>,
    pub publish_date: Option<String>,
    pub size: Option<u32>,
    pub status: Option<String>,
    pub stream_id: Option<u32>,
    pub suffix: Option<String>,
    pub transcoded_content_type: Option<String>,
    pub transcoded_suffix: Option<String>,
    pub r#type: Option<String>,
    pub username: Option<String>,
    pub year: Option<u32>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolder {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub id: u64,
    pub name: String,
    pub owner: String,
    pub public: bool,
    pub song_count: u32,

    pub comment: Option<String>,
    pub cover_art: Option<String>,
    pub created: Option<String>,
    #[serde(rename = "$value")]
    pub entries: Option<Vec<Entrires>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayQueue {
    current: u64,
    position: u64,
    username: String,

    changed: Option<String>,
    changed_by: Option<String>,
    #[serde(rename = "$value")]
    entries: Option<Vec<Media>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Radio {
    pub id: u64,
    pub name: String,
    pub stream_url: String,
    pub home_page_url: Option<String>,
}
#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    offset: Option<u32>,
    total: Option<u32>,
    #[serde(rename = "$value")]
    results: Option<Vec<Media>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    pub created: String,
    pub id: u64,
    pub url: String,
    pub username: String,
    pub visit_count: u32,

    pub description: Option<String>,
    pub last_visited: Option<String>,
    pub expires: Option<String>,
    #[serde(rename = "$value")]
    pub entries: Option<Vec<Media>>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub admin_role: bool,
    pub comment_role: bool,
    pub cover_art_role: bool,
    pub download_role: bool,
    pub jukebox_role: bool,
    pub playlist_role: bool,
    pub podcast_role: bool,
    pub scrobbling_enabled: bool,
    pub settings_role: bool,
    pub share_role: bool,
    pub stream_role: bool,
    pub upload_role: bool,
    pub username: String,

    pub email: Option<String>,
    #[serde(rename = "$value")]
    pub folders: Option<Vec<u32>>,
}
#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfo {
    id: u64,
    #[serde(rename = "$value")]
    properties: Option<Vec<VideoProperties>>,
}

#[cfg(test)]
mod test {
    use super::*;
    macro_rules! des {
        ($xml:expr) => {
            serde_xml_rs::from_str::<SubsonicResp>($xml).unwrap()
        };
    }

    use std::env::current_dir;
    use std::fs::*;
    use std::io::prelude::*;

    #[test]
    fn deser_tests() {
        // download all examples: wget http://www.subsonic.org/pages/api.jsp -r -A .xml
        // when running this, make sure to use -- --nocapture
        let files = read_dir(&*(current_dir().unwrap().to_string_lossy() + "/test"))
            .unwrap()
            .filter_map(|x| {
                x.ok().and_then(|r| {
                    r.path()
                        .file_name()
                        .and_then(|out| out.to_str().map(|s| s.to_string()))
                })
            });
        for file in files {
            println!("deserializing {}...", file);
            println!("---------------------------------------------------------------------");
            let mut out = "".to_string();
            let mut inp = File::open("./test/".to_string() + &file).unwrap();
            inp.read_to_string(&mut out).unwrap();
            println!("{:#?}", des!(&out));
            println!("---------------------------------------------------------------------\n");
        }
    }
}
