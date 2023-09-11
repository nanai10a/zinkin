pub type DateTime = chrono::DateTime<chrono::FixedOffset>;

pub struct Post {
    pub id: u32,
    pub content: String,
    pub posted_at: DateTime,
    pub created_at: DateTime,
    pub is_deleted: bool,
}

impl Post {
    pub fn new(id: u32, content: String, now: DateTime) -> Self {
        Self {
            id,
            content,
            posted_at: now,
            created_at: now,
            is_deleted: false,
        }
    }
}
