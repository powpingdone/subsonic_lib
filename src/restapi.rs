use super::*;
use const_format::{map_ascii_case, Case};
use deserialize::SubsonicResp;

macro_rules! api {
    ( $name:ident $maj:literal $min:literal $bug:literal
      $($pname:ident $amaj:literal $amin:literal $abug:literal : $nept:ty),* ) => {
        pub async fn $name(&self, $($pname:$nept,)*) -> anyhow::Result<SubsonicResp> {
            // server apis are version specifc
            #[allow(unused_comparisons)]
            if server_req!(self, $maj, $min, $bug) {
                let base = self.make_url();

                // rest api call name
                // it's always a camelcase version of the function name
                // ex: get_genres -> getGenres
                // so create that using a macro
                const NAME: &str = map_ascii_case!(Case::Camel, stringify!($name));

                // args (re)construction
                // turn args into the parseable part of the url
                #[allow(unused_mut)]
                let mut args = String::new();
                // for each arg
                $({
                    // turn into a Some() if the value isnt a Option
                    let param = IntoOptionHelper($pname).into_option();
                    if let Some(exist) = param {
                        #[allow(unused_comparisons)]
                        if server_req!(self, $amaj, $amin, $abug) {
                            // push the arg onto the url
                            args.push_str("&");
                            args.push_str(map_ascii_case!(Case::Camel, stringify!($pname)));
                            args.push_str("=");
                            args.push_str(&format!("{}", exist));
                        } else {
                            // note that args are *also* version specific
                            return Err(anyhow::Error::new(Error::ArgsVersionMismatch(
                                self.ver_major,
                                self.ver_minor,
                                self.ver_bugfix,
                                $amaj,
                                $amin,
                                $abug,
                            )));
                        }
                    }
                })*
                // and then make the request
                Ok(self.make_req(base.0 + NAME + &base.1 + &args).await?)
            } else {
                // you mismatched the api to your connected server
                Err(anyhow::Error::new(Error::APIVersionMismatch(
                    self.ver_major,
                    self.ver_minor,
                    self.ver_bugfix,
                    $maj,
                    $min,
                    $bug,
                )))
            }
        }
    };
}

// thanks Kestrer#9695 !
// this reliably transfers something into a option if needed
// used for constructing params
struct IntoOptionHelper<T>(T);

impl<T> IntoOptionHelper<Option<T>> {
    fn into_option(self) -> Option<T> {
        self.0
    }
}

trait IntoOptionFallback<T> {
    fn into_option(self) -> Option<T>;
}

impl<T> IntoOptionFallback<T> for IntoOptionHelper<T> {
    fn into_option(self) -> Option<T> {
        Some(self.0)
    }
}

impl SubsonicClient {
    api!(ping 1 0 0);
    api!(get_license 1 0 0);
    api!(get_music_folders 1 0 0);
    api!(get_indexes 1 0 0
        music_folder_id 1 0 0   : Option<String>,
        if_modified_since 1 0 0 : Option<u64>
    );
    api!(get_music_directory 1 0 0
        id 1 0 0 : String
    );
    api!(get_genres 1 9 0);
    api!(get_artists 1 8 0
        music_folder_id 1 8 0 : Option<String>
    );
    api!(get_artist 1 8 0
        id 1 8 0 : String
    );
    api!(get_album 1 8 0
        id 1 8 0 : String
    );
    api!(get_song 1 8 0
        id 1 8 0 : String
    );
    api!(get_videos 1 8 0);
    api!(get_video_info 1 14 0
        id 1 14 0 : String
    );
    api!(get_artist_info 1 11 0
        id 1 11 0                  : String,
        count 1 11 0               : Option<u32>,
        include_not_present 1 11 0 : Option<bool>
    );
    api!(get_artist_info2 1 11 0
        id 1 11 0                  : String,
        count 1 11 0               : Option<u32>,
        include_not_present 1 11 0 : Option<bool>
    );
    api!(get_album_info 1 14 0
        id 1 14 0 : String
    );
    api!(get_album_info2 1 14 0
        id 1 14 0 : String
    );
    api!(get_similar_songs 1 11 0
        id 1 11 0    : String,
        count 1 11 0 : Option<u32>
    );
    api!(get_similar_songs2 1 11 0
        id 1 11 0    : String,
        count 1 11 0 : Option<u32>
    );
    api!(get_top_songs 1 13 0
        artist 1 13 0 : String,
        count 1 13 0  : Option<u32>
    );
    api!(get_random_songs 1 2 0
        size 1 2 0            : Option<u32>,
        genre 1 2 0           : Option<String>,
        from_year 1 2 0       : Option<u32>,
        to_year 1 2 0         : Option<u32>,
        music_folder_id 1 2 0 : Option<String>
    );
    api!(get_songs_by_genre 1 9 0
        genre 1 9 0            : String,
        count 1 9 0            : Option<u32>,
        offset 1 9 0           : Option<u32>,
        music_folder_id 1 12 0 : Option<String>
    );
    api!(get_now_playing 1 0 0);
    api!(get_starred 1 8 0
        music_folder_id 1 12 0 : Option<String>
    );
    api!(get_starred2 1 8 0
        music_folder_id 1 12 0 : Option<String>
    );
    api!(search 1 0 0
        artist 1 0 0     : Option<String>,
        album 1 0 0      : Option<String>,
        title 1 0 0      : Option<String>,
        any 1 0 0        : Option<String>,
        count 1 0 0      : Option<u32>,
        offset 1 0 0     : Option<u32>,
        newer_than 1 0 0 : Option<u64>
    );
    api!(search2 1 4 0
        query 1 4 0            : String,
        artist_count 1 4 0     : Option<u32>,
        artist_offset 1 4 0    : Option<u32>,
        album_count 1 4 0      : Option<u32>,
        album_offset 1 4 0     : Option<u32>,
        song_count 1 4 0       : Option<u32>,
        song_offset 1 4 0      : Option<u32>,
        music_folder_id 1 12 0 : Option<String>
    );
    api!(search3 1 4 0
        query 1 4 0            : String,
        artist_count 1 4 0     : Option<u32>,
        artist_offset 1 4 0    : Option<u32>,
        album_count 1 4 0      : Option<u32>,
        album_offset 1 4 0     : Option<u32>,
        song_count 1 4 0       : Option<u32>,
        song_offset 1 4 0      : Option<u32>,
        music_folder_id 1 12 0 : Option<String>
    );
    api!(get_playlists 1 0 0
        username 1 8 0 : Option<String>
    );
    api!(get_playlist 1 0 0
        id 1 0 0 : String
    );
    api!(delete_playlist 1 2 0
        id 1 2 0 : String
    );
    api!(get_captions 1 14 0
        id 1 14 0     : String,
        format 1 14 0 : Option<CaptionType>
    );
    api!(get_cover_art 1 0 0
        id 1 0 0   : String,
        size 1 0 0 : Option<String> // in format DIMxDIM
    );
    api!(set_rating 1 6 0
        id 1 6 0     : String,
        rating 1 6 0 : u32);
    api!(scrobble 1 5 0
        id 1 5 0         : String,
        time 1 8 0       : Option<u64>,
        submission 1 5 0 : Option<bool>
    );
    api!(get_shares 1 6 0);
    api!(create_share 1 6 0
        id 1 6 0          : String,
        description 1 6 0 : Option<String>,
        expires 1 6 0     : Option<u64>);
    api!(update_share 1 6 0
        id 1 6 0          : String,
        description 1 6 0 : Option<String>,
        expires 1 6 0     : Option<u64>);
    api!(delete_share 1 6 0
        id 1 6 0 : String
    );
    api!(get_podcasts 1 6 0
        include_episodes 1 9 0 : Option<bool>,
        id 1 9 0               : Option<String>
    );

    api!(get_users 1 8 0);

    // manual implimentation
    // get_album_list, get_album_list2, create_playlist,
    // update_playlist, stream, download, hls, get_avatar,
    // star, unstar
    // note that the multiparam stuff could be automated
}
