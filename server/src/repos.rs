use sqlx::FromRow;

#[allow(clippy::wildcard_imports)]
use crate::utils::*;
use crate::{models, rows};

pub trait PostRepository {
    async fn all(&self) -> anyhow::Result<Vec<models::Post>>;
    async fn find_one(&self, id: u32) -> anyhow::Result<Option<models::Post>>;
    async fn find_all(&self, id: u32) -> anyhow::Result<Vec<models::Post>>;
    async fn create(&self, model: models::Post) -> anyhow::Result<()>;
    async fn update(
        &self,
        id: u32,
        content: String,
        created_at: models::DateTime,
    ) -> anyhow::Result<()>;
    async fn delete(&self, id: u32) -> anyhow::Result<()>;
    async fn restore(&self, id: u32) -> anyhow::Result<()>;
}

pub struct SqlitePostRepository(sqlx::SqlitePool);

impl SqlitePostRepository {
    pub async fn new(p: &std::path::Path) -> anyhow::Result<Self> {
        if !std::fs::try_exists(p)? {
            std::fs::write(p, [])?;
        }

        let url = "sqlite://".to_string() + &p.to_string_lossy();

        let db = sqlx::SqlitePool::connect(&url).await?;
        sqlx::migrate!().run(&db).await?;

        Ok(Self(db))
    }
}

impl core::ops::Deref for SqlitePostRepository {
    type Target = sqlx::SqlitePool;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl PostRepository for SqlitePostRepository {
    async fn all(&self) -> anyhow::Result<Vec<models::Post>> {
        #[rustfmt::skip]
        const QUERY: &str = "SELECT p.*, pf.flags FROM posts AS p \
                             JOIN post_flags AS pf ON p.id = pf.id \
                             WHERE DATETIME(p.created_at) = (
                                 SELECT MAX(DATETIME(created_at)) FROM posts
                                 WHERE id = p.id
                             )
                             ORDER BY DATETIME(created_at) DESC, p.id DESC";

        let models = sqlx::query(QUERY)
            .fetch_all(&**self)
            .await?
            .iter()
            .map(rows::Post::from_row)
            .try_map(rows::Post::into_model)
            .map(|r| r.map_err(anyhow::Error::new).flatten())
            .try_collect::<Vec<_>>()?;

        Ok(models)
    }

    async fn find_one(&self, id: u32) -> anyhow::Result<Option<models::Post>> {
        #[rustfmt::skip]
        const QUERY: &str = "SELECT p.*, pf.flags FROM posts AS p \
                             JOIN post_flags AS pf ON p.id = pf.id \
                             WHERE p.id = ? \
                             ORDER BY DATETIME(p.created_at) DESC \
                             LIMIT 1";

        let model = match sqlx::query(QUERY)
            .bind(id.to_be_bytes().to_vec())
            .fetch_one(&**self)
            .await
        {
            Ok(ref row) => rows::Post::from_row(row)?.into_model()?,
            Err(sqlx::Error::RowNotFound) => return Ok(None),
            Err(err) => anyhow::bail!(err),
        };

        Ok(Some(model))
    }

    async fn find_all(&self, id: u32) -> anyhow::Result<Vec<models::Post>> {
        #[rustfmt::skip]
        const QUERY: &str = "SELECT p.*, pf.flags FROM posts AS p \
                             JOIN post_flags AS pf ON p.id = pf.id \
                             WHERE p.id = ? \
                             ORDER BY DATETIME(p.created_at) DESC";

        let models = sqlx::query(QUERY)
            .bind(id.to_be_bytes().to_vec())
            .fetch_all(&**self)
            .await?
            .iter()
            .map(rows::Post::from_row)
            .try_map(rows::Post::into_model)
            .map(|r| r.map_err(anyhow::Error::new).flatten())
            .try_collect::<Vec<_>>()?;

        Ok(models)
    }

    async fn create(&self, model: models::Post) -> anyhow::Result<()> {
        #[rustfmt::skip]
        const QUERY_0: &str = "INSERT INTO posts (id, content, posted_at, created_at) \
                               VALUES (?, ?, ?, ?)";

        #[rustfmt::skip]
        const QUERY_1: &str = "INSERT INTO post_flags (id, flags) \
                               VALUES (?, ?)";

        let row = rows::Post::from_model(model)?;

        let result = sqlx::query(QUERY_0)
            .bind(row.id)
            .bind(row.content)
            .bind(row.posted_at)
            .bind(row.created_at)
            .execute(&**self)
            .await?;

        if result.rows_affected() != 1 {
            anyhow::bail!("failed to insert into posts");
        }

        let result = sqlx::query(QUERY_1)
            .bind(row.id)
            .bind(row.flags)
            .execute(&**self)
            .await?;

        if result.rows_affected() != 1 {
            anyhow::bail!("failed to insert into post_flags");
        }

        Ok(())
    }

    async fn update(
        &self,
        id: u32,
        content: String,
        created_at: models::DateTime,
    ) -> anyhow::Result<()> {
        #[rustfmt::skip]
        const QUERY: &str = "INSERT INTO posts (id, content, posted_at, created_at) \
                             SELECT id, ?, posted_at, ? FROM posts \
                             WHERE id = ?";

        let result = sqlx::query(QUERY)
            .bind(content)
            .bind(created_at)
            .bind(id)
            .execute(&**self)
            .await?;

        if result.rows_affected() != 1 {
            anyhow::bail!("failed to insert into posts");
        }

        Ok(())
    }

    async fn delete(&self, id: u32) -> anyhow::Result<()> {
        #[rustfmt::skip]
        const QUERY: &str = "UPDATE post_flags \
                             SET flags = flags | 0b0001 \
                             WHERE id = ?";

        let result = sqlx::query(QUERY).bind(id).execute(&**self).await?;

        if result.rows_affected() != 1 {
            anyhow::bail!("failed to update post_flags");
        }

        Ok(())
    }

    async fn restore(&self, id: u32) -> anyhow::Result<()> {
        #[rustfmt::skip]
        const QUERY: &str = "UPDATE post_flags \
                             SET flags = flags ^ 0b0001 \
                             WHERE id = ?";

        let result = sqlx::query(QUERY).bind(id).execute(&**self).await?;

        if result.rows_affected() != 1 {
            anyhow::bail!("failed to update post_flags");
        }

        Ok(())
    }
}
