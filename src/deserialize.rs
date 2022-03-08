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
    MediaList(Vec<Media>),

    #[serde(alias = "searchResult2")]
    #[serde(alias = "searchResult3")]
    #[serde(alias = "starred")]
    #[serde(alias = "starred2")]
    GenericList(Option<Vec<SubsonicInfo>>),

    Album(Album),
    #[serde(alias = "albumList2")]
    AlbumList(Vec<Album>),
    Artist(Artist),
    Bookmarks(Vec<Bookmark>),
    ChatMessages(Vec<ChatMessage>),
    Genres(Vec<Genre>),
    InternetRadioStations(Vec<Radio>),
    MusicFolders(Vec<MusicFolder>),
    Playlist(Playlist),
    Playlists(Vec<Playlist>),
    Podcasts(Vec<Channel>),
    Shares(Vec<Share>),
    Song(Media),
    User(User),
    Users(Vec<User>),

    AlbumInfo {
        notes: Option<String>,
        music_brainz_id: Option<String>,
        last_fm_url: Option<String>,
        small_image_url: Option<String>,
        medium_image_url: Option<String>,
        large_image_url: Option<String>,
    },
    #[serde(alias = "artistInfo2")]
    ArtistInfo {
        music_brainz_id: Option<String>,
        last_fm_url: Option<String>,
        small_image_url: Option<String>,
        medium_image_url: Option<String>,
        large_image_url: Option<String>,
        biography: Option<String>,
        // this is implimented for the "similarArtists" at the end
        #[serde(rename = "similarArtists")]
        similar_artists: Option<Vec<Artist>>,
    },
    Artists{
        ignored_articles: Option<String>,
        #[serde(rename = "$value")]
        indexes: Vec<Index>
    },
    Directory {
        id: u64,
        parent: u64,
        name: String,
        starred: Option<String>,
        #[serde(rename = "$value")]
        children: Vec<Media>
    },
    Error {
        code: u32,
        message: String,
    },
    Indexes {
        last_modified: u64,
        ignored_articles: Option<String>,
        #[serde(rename = "$value")]
        list: Vec<Indexes>
    },
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
    PlayQueue {
        current: u64,
        position: u64,
        username: String,

        changed: Option<String>,
        changed_by: Option<String>,
        #[serde(rename = "$value")]
        entries: Option<Vec<Media>>,
    },
    ScanStatus {
        scanning: bool,

        count: Option<u32>,
    },
    SearchResult {
        offset: Option<u32>,
        total: Option<u32>,
        #[serde(rename = "$value")]
        results: Option<Vec<Media>>,
    },
    VideoInfo {
        id: u64,
        #[serde(rename = "$value")]
        properties: Option<Vec<VideoProperties>>,
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
pub enum VideoProperties {
    AudioTrack {
        id: u64,
        name: String,
        language_code: String,
    },
    Captions {
        id: u64,
        name: String,
    },
    Conversion {
        id: u64,
        bitrate: u32,
    }
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
    pub year: Option<u32>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolder {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
       pub id: u64,
      pub  name: String,
       pub owner: String,
      pub  public: bool,
    pub    song_count: u32,

pub        comment: Option<String>,
  pub      cover_art: Option<String>,
    pub created: Option<String>,
    #[serde(rename = "$value")]
    pub entries: Option<Vec<Entrires>>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Radio {
    pub id: u64,
    pub name: String,
    pub stream_url: String,
    pub home_page_url: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
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

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub admin_role: bool,
    pub comment_role: bool,
    pub cover_art_role:  bool,
    pub download_role: bool,
    pub jukebox_role:bool,
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

// download all examples: wget http://www.subsonic.org/pages/api.jsp -r -A .xml
