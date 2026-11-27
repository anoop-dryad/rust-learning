#[derive(Debug)]
pub struct Book {
    pub title: String,
    author: String,
}

impl Book {
    pub fn new(title: &str, author: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
        }
    }

    pub fn get_author(&self) -> &str {
        &self.author
    }
}
