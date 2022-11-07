use std::fs;
use ytbot::*;
// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    mod get_list_of_videos {
        use super::*;
        async fn init() -> Vec<VideoInfo> {
            let query = "#fyp";

            let list = get_list(query).await;

            return list;
        }
        #[tokio::test]
        async fn list_length_is_greater_than_zero() {
            let list = init().await;
            // get_list returns an array with a length greater than 0
            assert_ne!(
                list.len(),
                0,
                "get_list returns an array with a length greater than 0"
            );
        }

        #[tokio::test]
        async fn author_names_are_not_empty() {
            let list = init().await;
            // every authors name is longer than 0
            list.iter().for_each(|video| {
                assert_ne!(
                    video.author.len(),
                    0,
                    "some authors name is not longer than 0"
                );
            });
        }
        #[tokio::test]
        async fn video_ids_are_not_empty() {
            let list = init().await;
            // every authors name is longer than 0
            list.iter().for_each(|video| {
                assert_ne!(video.id.len(), 0, "some video ids are empty");
            });
        }
        #[tokio::test]
        async fn video_descriptions_are_not_empty() {
            let list = init().await;
            // every authors name is longer than 0
            list.iter().for_each(|video| {
                assert_ne!(video.id.len(), 0, "some video descriptions are empty");
            });
        }
    }
    mod download_file {
        use super::*;
        async fn init() -> Vec<VideoInfo> {
            let query = "#fyp";
            let list = get_list(query).await;
            return list;
        }
        #[tokio::test]
        async fn download_file() {
            let list = init().await;
            let video = list.get(0).unwrap();
            let path = download_video(&video.url, "test.mp4").await;
            assert_eq!(path, "test.mp4");
        }

        #[tokio::test]
        async fn file_is_not_empty() {
            let list = init().await;
            let video = list.get(0).unwrap();
            let path = download_video(&video.url, "test.mp4").await;
            let metadata = fs::metadata(path).unwrap();
            let size = metadata.len();
            assert_ne!(size, 0);
        }
    }
}
