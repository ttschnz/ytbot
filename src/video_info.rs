use std::fmt;

#[derive(Clone, Debug)]
pub struct VideoInfo {
    pub id: String,
    pub url: String,
    pub desc: String,
    pub author: String,
}

// implement the Display trait for VideoInfo
impl fmt::Display for VideoInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}\nURL: {}\nDescription: {}\nAuthor: {}\n",
            self.id, self.url, self.desc, self.author
        )
    }
}
