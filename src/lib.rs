mod crawler;
mod video_info;

pub use self::crawler::download_video;
pub use self::crawler::get_list;
pub use self::video_info::VideoInfo;
use std::env;

pub fn get_config() -> (String, String) {
    let query = env::var("SEARCH_QUERY").unwrap_or_else(|_| "#trending".into());
    let path = env::var("OUT_PATH").unwrap_or_else(|_| "test.mp4".into());
    return (query, path);
}
