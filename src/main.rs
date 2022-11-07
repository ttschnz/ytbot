use ytbot::download_video;
use ytbot::get_list;

#[tokio::main]
async fn main() {
    let query = "#fyp";
    let list = get_list(query).await;
    for video in &list {
        println!("{}", video);
    }

    let video = &list[0];
    let path = "test.mp4";
    download_video(&video.url, path).await;
    println!("done downloading video: ./{}", path);
}
