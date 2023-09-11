use serde::{Deserialize, Serialize};

#[allow(clippy::wildcard_imports)]
use crate::utils::*;

pub type DateTime = chrono::DateTime<chrono::FixedOffset>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    // not modified
    pub id: u32,
    // not modified
    pub content: String,
    #[serde(with = "datetime")]
    pub posted_at: DateTime,
    #[serde(with = "datetime")]
    pub created_at: DateTime,
    // not modified
    pub is_deleted: bool,
}

mod datetime {
    use serde::{Deserialize, Deserializer, Serializer};

    use super::DateTime;

    pub fn serialize<S: Serializer>(date: &DateTime, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&date.to_rfc3339())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime, D::Error> {
        let str = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&str).map_err(serde::de::Error::custom)
    }
}

impl IntoModel for Post {
    type Model = crate::models::Post;

    fn into_model(self) -> anyhow::Result<Self::Model> {
        let Self {
            id,
            content,
            posted_at,
            created_at,
            is_deleted,
        } = self;

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

    fn from_model(model: Self::Model) -> anyhow::Result<Self>
    where Self: Sized {
        let Self::Model {
            id,
            content,
            posted_at,
            created_at,
            is_deleted,
        } = model;

        Ok(Self {
            id,
            content,
            posted_at,
            created_at,
            is_deleted,
        })
    }
}
