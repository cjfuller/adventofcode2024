pub mod coords;
pub mod parsers;

use std::fmt::Debug;

pub struct ChunkedIter<const N: usize, T: Iterator> {
    underlying: T,
}

impl<const N: usize, T: Iterator> ChunkedIter<N, T> {
    fn new(underlying: T) -> Self {
        assert!(N > 0);
        Self { underlying }
    }
}

pub enum Chunk<const N: usize, I> {
    Complete([I; N]),
    Partial([Option<I>; N]),
}

impl<const N: usize, I: Debug> Debug for Chunk<N, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chunk::Complete(inner) => f.debug_tuple("Chunk::Complete").field(inner).finish(),
            Chunk::Partial(inner) => f.debug_tuple("Chunk::Partial").field(inner).finish(),
        }
    }
}

impl<const N: usize, T: Iterator> Iterator for ChunkedIter<N, T> {
    type Item = Chunk<N, T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.underlying.next() {
            let mut n_once = Some(n);
            let next_arr = std::array::from_fn(|idx| {
                if idx == 0 {
                    Some(n_once.take().unwrap())
                } else {
                    assert!(idx < N);
                    self.underlying.next()
                }
            });
            if next_arr.iter().all(|it| it.is_some()) {
                Some(Chunk::Complete(next_arr.map(|it| it.unwrap())))
            } else {
                Some(Chunk::Partial(next_arr))
            }
        } else {
            None
        }
    }
}

pub trait IntoChunkedIter: Iterator + Sized {
    fn into_chunked<const N: usize>(self) -> ChunkedIter<N, Self>;
}

impl<I: Iterator + Sized> IntoChunkedIter for I {
    fn into_chunked<const N: usize>(self) -> ChunkedIter<N, Self> {
        ChunkedIter::new(self)
    }
}
