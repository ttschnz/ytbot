use super::VideoInfo;
use json;
use regex::Regex;
use reqwest;
use std::fs;
use std::io::Write;

/**
 * Gets a list of videos ready to download
 */
pub async fn get_list(query: &str) -> Vec<VideoInfo> {
    let reg: regex::Regex =
        Regex::new(r"<script [^>]*SIGI_STATE[^>]*>(?P<json>[^<]*)</script>").unwrap();

    let url: String = format!(r"https:\/\/www.tiktok.com/{}", query);
    let body = reqwest::get(url)
        .await
        .unwrap_or_else(|_| {
            panic!("Failed to get list of videos");
        })
        .text()
        .await
        .unwrap_or_else(|_| {
            panic!("Failed to get list of videos");
        });
    // print!("body: {}", body);

    let caps = reg.captures(&body).unwrap();
    let json_data = caps.name("json").unwrap().as_str();

    let parsed = json::parse(json_data).unwrap();

    let mut list: Vec<VideoInfo> = [].to_vec();

    parsed["ItemModule"].entries().for_each(|video| {
        let id = video.1["id"].to_string();
        let url = video.1["video"]["downloadAddr"].to_string();
        let desc = video.1["desc"].to_string();
        let author = video.1["author"].to_string();
        list.push(VideoInfo {
            id,
            url,
            desc,
            author,
        });
    });

    return list;
}

/**
 * Downloads a video from a url to a path
 */

pub async fn download_video<'a>(url: &str, path: &'a str) -> &'a str {
    let mut resp = reqwest::get(url).await.unwrap();
    let mut file = fs::File::create(path).unwrap();
    while let Some(chunk) = resp.chunk().await.unwrap() {
        file.write_all(&chunk).unwrap();
    }
    return path;
}
