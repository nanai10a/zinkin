#[allow(clippy::wildcard_imports)]
use crate::utils::*;

type DateTime = chrono::DateTime<chrono::FixedOffset>;

#[derive(sqlx::FromRow)]
pub struct Post {
    pub id: u32,
    pub content: String,
    pub posted_at: DateTime,
    pub created_at: DateTime,
    pub flags: u32,
}

impl IntoModel for Post {
    type Model = crate::models::Post;

    fn into_model(self) -> anyhow::Result<Self::Model> {
        let Self {
            id,
            content,
            posted_at,
            created_at,
            flags,
        } = self;

        let is_deleted = flags & 0b0001 != 0;

        Ok(Self::Model {
            id,
            content,
            posted_at,
            created_at,
            is_deleted,
        })
    }
}

impl FromModel for Post {
    type Model = crate::models::Post;

    #[allow(clippy::identity_op)]
    fn from_model(model: Self::Model) -> anyhow::Result<Self>
    where Self: Sized {
        let Self::Model {
            id,
            content,
            posted_at,
            created_at,
            is_deleted,
        } = model;

        let mut flags = 0;

        flags |= u32::from(is_deleted) << 0;

        Ok(Self {
            id,
            content,
            posted_at,
            created_at,
            flags,
        })
    }
}
