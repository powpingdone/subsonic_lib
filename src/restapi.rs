use super::*;
use const_format::{map_ascii_case, Case};
use deserialize::SubsonicResp;

macro_rules! api {
    ( $name:ident $maj:literal $min:literal $bug:literal $($pname:ident $amaj:literal $amin:literal $abug:literal : $nept:ty),* ) => {
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
                $(
                    {
                        // turn into a Some() if the value isnt a Some()
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
                    }
                )*
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
        music_folder_id 1 0 0 : Option<String>,
        if_modified_since 1 0 0 : Option<u64>);
    api!(get_music_directory 1 0 0
        id 1 0 0 : String);
    api!(get_genres 1 9 0);

    api!(get_videos 1 8 0);
    api!(get_now_playing 1 0 0);
    api!(get_users 1 8 0);
}
