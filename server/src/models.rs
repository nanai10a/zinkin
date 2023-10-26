pub trait IntoModel {
    type Model;

    fn into_model(self) -> anyhow::Result<Self::Model>;
}

pub trait FromModel {
    type Model;

    fn from_model(model: Self::Model) -> anyhow::Result<Self>
    where Self: Sized;
}

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
