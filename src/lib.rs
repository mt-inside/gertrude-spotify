wai_bindgen_rust::export!("plugin.wai");

use url::Url;

pub struct Plugin;

impl plugin::Plugin for Plugin {
    //"https://open.spotify.com/album/3HmGULnPeH7ZhDkQlDibh3?si=H_nxP4YkTAKM1SnVvc2VWg"
    //Need an API key to call spotify: https://developer.spotify.com/documentation/web-api

    // TODO: returning Vecs gives out-of-bound string read errors
    fn handle_privmsg(msgs: Vec<String>) -> Option<String> {
        if let Ok(u) = Url::parse(&msgs[msgs.len() - 1]) {
            if u.host_str() == Some("open.spotify.com") {
                let segs = u.path_segments().unwrap().collect::<Vec<&str>>();

                return Some(format!("Artist: {}; {}: {}", segs[1], segs[0], "foo"));
            }
        }

        None
    }
}
