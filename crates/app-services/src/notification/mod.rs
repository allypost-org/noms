use crate::Tag;

#[derive(Debug, Clone)]
pub struct Notification {
    pub title: Option<String>,
    pub message: String,
    pub tags: Vec<Tag>,
}
