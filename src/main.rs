use ytbot::get_list;

#[tokio::main]
async fn main() {
    let query = "#fyp";
    let list = get_list(query).await;
    for video in list {
        println!("{}", video);
    }
}
