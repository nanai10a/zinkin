type DateTime = chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub content: String,
    pub posted_at: DateTime,
    pub created_at: DateTime,
    pub is_deleted: bool,
}

impl crate::models::IntoModel for Post {
    type Model = crate::models::Post;

    fn into_model(self) -> anyhow::Result<Self::Model> {
        let Self {
            id,
            content,
            posted_at,
            created_at,
            is_deleted,
        } = self;

        let id = id as u32;

        Ok(Self::Model {
            id,
            content,
            posted_at,
            created_at,
            is_deleted,
        })
    }
}

impl crate::models::FromModel for Post {
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

        let id = id as i64;

        Ok(Self {
            id,
            content,
            posted_at,
            created_at,
            is_deleted,
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct Key {
    pub content: Vec<u8>,
}

impl crate::models::IntoModel for Key {
    type Model = webauthn_rs::prelude::Passkey;

    fn into_model(self) -> anyhow::Result<Self::Model> {
        let Self { content } = self;

        Ok(rmp_serde::from_slice(&content)?)
    }
}

impl crate::models::FromModel for Key {
    type Model = webauthn_rs::prelude::Passkey;

    fn from_model(model: Self::Model) -> anyhow::Result<Self>
    where Self: Sized {
        let content = rmp_serde::to_vec(&model)?;

        Ok(Self { content })
    }
}
