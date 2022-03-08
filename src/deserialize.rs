use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubsonicResp {
    pub status: String,
    pub version: String,
    #[serde(rename = "$value")]
    pub resp: SubsonicInfo,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SubsonicInfo {
    Error {
        code: u32,
        message: String,
    },
    Album(Album),
    AlbumInfo {
        notes: Option<String>,
        music_brainz_id: Option<String>,
        last_fm_url: Option<String>,
        small_image_url: Option<String>,
        medium_image_url: Option<String>,
        large_image_url: Option<String>,
    },
    #[serde(alias = "albumList2")]
    AlbumList(Vec<Album>),
    Artist(Artist),
    #[serde(alias = "artistInfo2")]
    ArtistInfo {
        music_brainz_id: Option<String>,
        last_fm_url: Option<String>,
        small_image_url: Option<String>,
        medium_image_url: Option<String>,
        large_image_url: Option<String>,
        biography: Option<String>,
        // this is implimented for the "similarArtists" at the end
        // #[serde(rename = "similarArtists")]
        // similar_artists: Option<Vec<SubsonicArtist>>,
    },
    Artists{
        ignored_articles: Option<String>,
        #[serde(rename = "$value")]
        indexes: Vec<Index>
    },
    Bookmarks(Vec<Bookmark>),
    ChatMessages(Vec<ChatMessage>),
    Directory {
        id: u64,
        parent: u64,
        name: String,
        starred: Option<String>,
        #[serde(rename = "$value")]
        children: Vec<Media>
    },
    Genres(Vec<Genre>),
    Indexes {
        last_modified: u64,
        ignored_articles: Option<String>,
        #[serde(rename = "$value")]
        list: Vec<Indexes>
    },
    InternetRadioStations(Vec<Radio>),
    JukeboxPlaylist{
        current_index: u64,
        playing: bool,
        gain: f32,
        position: u64,
        #[serde(rename = "$value")]
        list: Vec<Media>,
    },
    JukeboxStatus{
        current_index: u64,
        playing: bool,
        gain: f32,
        position: u64,
    },
    License {
        valid: bool,
        email: String,
        license_expires: String,
    },
    Lyrics {
        artist: String,
        title: String,
        #[serde(rename = "$value")]
        lyrics: String,
    },
    MusicFolders(Vec<MusicFolder>),
    NewestPodcasts(Vec<Media>),
    NowPlaying(Vec<Media>),
    Playlist {
        duration: u64,
        id: u64,
        name: String,
        owner: String,
        public: bool,
        song_count: u32,

        comment: Option<String>,
        cover_art: Option<String>,
        created: Option<String>,
        #[serde(rename = "$value")]
        entries: Vec<Entrires>,
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
pub enum Indexes {
    Shortcut {
        id: u64,
        name: String,
    },
    Index(Index),
    Child(Media),
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: u64,
    pub name: String,
    pub song_count: u32,
    pub duration: u64,
    pub created: String,
    pub user_rating: Option<String>,
    pub average_rating: Option<String>,
    pub parent: Option<u64>,
    pub cover_art: Option<String>,
    pub artist: Option<String>,
    pub artist_id: Option<u64>,
    #[serde(rename = "$value")]
    pub songs: Option<Vec<Media>>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub album_count: Option<u32>,
    pub cover_art: Option<String>,
    pub starred: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    pub position: u64,
    pub username: String,
    pub comment: Option<String>,
    pub created: Option<String>,
    pub changed: Option<String>,
    #[serde(rename = "$value")]
    pub entries: Vec<Media>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub username: String,
    pub time: u64,
    pub message: String,
}


#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub song_count: u32,
    pub album_count: u32,
    #[serde(rename = "$value")]
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub name: String,
    #[serde(rename = "$value")]
    pub artists: Vec<Artist>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: u64,
    pub parent: u64,
    pub title: String,
    pub is_dir: bool,

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
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolder {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Radio {
    pub id: u64,
    pub name: String,
    pub stream_url: String,
    pub home_page_url: Option<String>,
}

// download all examples: wget http://www.subsonic.org/pages/api.jsp -r -A .xml
