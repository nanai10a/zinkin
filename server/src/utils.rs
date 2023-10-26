use core::ops::{Residual, Try};

pub trait IteratorExt: Iterator {
    fn try_map<T, F>(
        self,
        mut f: F,
    ) -> impl Iterator<Item = <<Self::Item as Try>::Residual as Residual<T>>::TryType>
    where
        Self: Sized,
        Self::Item: Try,
        <Self::Item as Try>::Residual: Residual<T>,
        F: FnMut(<Self::Item as Try>::Output) -> T,
    {
        self.map(move |v| try { f(v?) })
    }
}

impl<I: Iterator> IteratorExt for I {}

#[test]
fn try_map() {
    let v = [1, 2, 3]
        .into_iter()
        .map(Ok::<_, core::convert::Infallible>)
        .try_map(|v| v + 1)
        .try_collect::<Vec<_>>();

    assert_eq!(v, Ok(vec![2, 3, 4]));
}
