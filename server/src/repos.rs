use sqlx::FromRow;

use crate::models::{FromModel, IntoModel};
use crate::utils::IteratorExt;
use crate::{models, rows};

mod ext {
    pub use webauthn_rs::prelude::Passkey;
}

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

pub struct PgRepository(sqlx::PgPool);

impl PgRepository {
    pub async fn new(p: impl AsRef<str>) -> anyhow::Result<Self> {
        let db = sqlx::PgPool::connect(p.as_ref()).await?;
        sqlx::migrate!().run(&db).await?;

        Ok(Self(db))
    }
}

impl core::ops::Deref for PgRepository {
    type Target = sqlx::PgPool;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl PostRepository for PgRepository {
    async fn all(&self) -> anyhow::Result<Vec<models::Post>> {
        #[rustfmt::skip]
        const QUERY: &str = "SELECT p.*, pf.is_deleted FROM posts AS p \
                             JOIN post_flags AS pf ON p.id = pf.id \
                             WHERE p.created_at = ( \
                                 SELECT MAX(created_at) FROM posts \
                                 WHERE id = p.id \
                             ) AND pf.is_deleted = FALSE \
                             ORDER BY created_at DESC, p.id DESC";

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
        const QUERY: &str = "SELECT p.*, pf.is_deleted FROM posts AS p \
                             JOIN post_flags AS pf ON p.id = pf.id \
                             WHERE p.id = $1 \
                             ORDER BY p.created_at DESC \
                             LIMIT 1";

        let model = match sqlx::query(QUERY).bind(id as i64).fetch_one(&**self).await {
            Ok(ref row) => rows::Post::from_row(row)?.into_model()?,
            Err(sqlx::Error::RowNotFound) => return Ok(None),
            Err(err) => anyhow::bail!(err),
        };

        Ok(Some(model))
    }

    async fn find_all(&self, id: u32) -> anyhow::Result<Vec<models::Post>> {
        #[rustfmt::skip]
        const QUERY: &str = "SELECT p.*, pf.is_deleted FROM posts AS p \
                             JOIN post_flags AS pf ON p.id = pf.id \
                             WHERE p.id = $1 \
                             ORDER BY p.created_at DESC";

        let models = sqlx::query(QUERY)
            .bind(id as i64)
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
                               VALUES ($1, $2, $3, $4)";

        #[rustfmt::skip]
        const QUERY_1: &str = "INSERT INTO post_flags (id, is_deleted) \
                               VALUES ($1, $2)";

        let rows::Post {
            id,
            content,
            posted_at,
            created_at,
            is_deleted,
        } = rows::Post::from_model(model)?;

        let result = sqlx::query(QUERY_0)
            .bind(id)
            .bind(content)
            .bind(posted_at)
            .bind(created_at)
            .execute(&**self)
            .await?;

        if result.rows_affected() != 1 {
            anyhow::bail!("failed to insert into posts");
        }

        let result = sqlx::query(QUERY_1)
            .bind(id)
            .bind(is_deleted)
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
                             SELECT id, $1, posted_at, $2 FROM posts \
                             WHERE id = $3";

        let result = sqlx::query(QUERY)
            .bind(content)
            .bind(created_at)
            .bind(id as i64)
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
                             SET is_deleted = TRUE \
                             WHERE id = $1";

        let result = sqlx::query(QUERY).bind(id as i64).execute(&**self).await?;

        if result.rows_affected() != 1 {
            anyhow::bail!("failed to update post_flags");
        }

        Ok(())
    }

    async fn restore(&self, id: u32) -> anyhow::Result<()> {
        #[rustfmt::skip]
        const QUERY: &str = "UPDATE post_flags \
                             SET is_deleted = FALSE \
                             WHERE id = $1";

        let result = sqlx::query(QUERY).bind(id as i64).execute(&**self).await?;

        if result.rows_affected() != 1 {
            anyhow::bail!("failed to update post_flags");
        }

        Ok(())
    }
}

pub trait KeyRepository {
    async fn all(&self) -> anyhow::Result<Vec<ext::Passkey>>;
    async fn get(&self, id: u32) -> anyhow::Result<Option<ext::Passkey>>;
    async fn push(&self, id: u32, model: ext::Passkey) -> anyhow::Result<()>;
    async fn remove(&self, id: u32) -> anyhow::Result<()>;
}

impl KeyRepository for PgRepository {
    async fn all(&self) -> anyhow::Result<Vec<ext::Passkey>> {
        #[rustfmt::skip]
        const QUERY: &str = "SELECT content FROM keys";

        let models = sqlx::query(QUERY)
            .fetch_all(&**self)
            .await?
            .iter()
            .map(rows::Key::from_row)
            .try_map(rows::Key::into_model)
            .map(|r| r.map_err(anyhow::Error::new).flatten())
            .try_collect::<Vec<_>>()?;

        Ok(models)
    }

    async fn get(&self, id: u32) -> anyhow::Result<Option<ext::Passkey>> {
        #[rustfmt::skip]
        const QUERY: &str = "SELECT content FROM keys WHERE id = $1";

        let model = match sqlx::query(QUERY).bind(id as i64).fetch_one(&**self).await {
            Ok(ref row) => rows::Key::from_row(row)?.into_model()?,
            Err(sqlx::Error::RowNotFound) => return Ok(None),
            Err(err) => anyhow::bail!(err),
        };

        Ok(Some(model))
    }

    async fn push(&self, id: u32, model: ext::Passkey) -> anyhow::Result<()> {
        #[rustfmt::skip]
        const QUERY: &str = "INSERT INTO keys (id, content) \
                             VALUES ($1, $2)";

        let rows::Key { content } = rows::Key::from_model(model)?;

        let result = sqlx::query(QUERY)
            .bind(id as i64)
            .bind(content)
            .execute(&**self)
            .await?;

        if result.rows_affected() != 1 {
            anyhow::bail!("failed to insert into keys");
        }

        Ok(())
    }

    async fn remove(&self, id: u32) -> anyhow::Result<()> {
        #[rustfmt::skip]
        const QUERY: &str = "DELETE FROM keys WHERE id = $1";

        let result = sqlx::query(QUERY).bind(id as i64).execute(&**self).await?;

        if result.rows_affected() != 1 {
            anyhow::bail!("failed to delete from keys");
        }

        Ok(())
    }
}
