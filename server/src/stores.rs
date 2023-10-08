use anyhow::Result;

pub trait Store<T> {
    type Key;

    async fn entry(&self, key: Self::Key) -> Result<impl Entry<T>>;
}

pub trait Entry<T> {
    async fn is_empty(&self) -> Result<bool>;

    async fn set(self, val: T) -> Result<bool>;
    async fn get(self) -> Result<Option<T>>;
}
