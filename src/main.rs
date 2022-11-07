use ytbot::download_video;
use ytbot::get_config;
use ytbot::get_list;

#[tokio::main]
async fn main() {
    let (query, path) = get_config();

    let list = get_list(&query).await;
    for video in &list {
        println!("{}", video);
    }

    let video = &list[0];
    download_video(&video.url, &path).await;
    println!("done downloading video: ./{}", path);
}
