pub type DateTime = chrono::DateTime<chrono::FixedOffset>;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: u32,
    pub content: PostContent,
    pub posted_at: DateTime,
    pub created_at: DateTime,
    pub is_deleted: bool,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostContent {
    src: String,
    html: String,
}

impl crate::models::FromModel for Post {
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

        let content = {
            use pulldown_cmark::{html, Parser};

            let mut html = String::new();
            html::push_html(&mut html, Parser::new(&content));

            PostContent { src: content, html }
        };

        Ok(Self {
            id,
            content,
            posted_at,
            created_at,
            is_deleted,
        })
    }
}
