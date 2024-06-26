use std::{fmt, iter::FusedIterator};

use engine_id_newtypes::{IdOperations, IdRange};

use crate::{PartialCacheRegistry, RegistryId};

/// Iterator for readers
///
/// T indicates the type that will be yielded by the Iterator
#[derive(Clone, Copy)]
pub struct Iter<'a, T>
where
    T: IdReader,
{
    range: IdRange<T::Id>,
    registry: &'a crate::PartialCacheRegistry,
}

impl<'a, T> Iter<'a, T>
where
    T: IdReader,
    T::Id: IdOperations,
{
    pub(crate) fn new(range: IdRange<T::Id>, registry: &'a PartialCacheRegistry) -> Self {
        Iter { range, registry }
    }
}

pub trait IdReader {
    type Id: RegistryId;
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: IdReader,
    T::Id: IdOperations,
{
    type Item = <T::Id as RegistryId>::Reader<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.range.next()?;

        Some(self.registry.read(next))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = IdOperations::distance(self.range.start, self.range.end);
        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T>
where
    T: IdReader,
    T::Id: IdOperations,
    IdRange<T::Id>: ExactSizeIterator,
{
}

impl<'a, T> FusedIterator for Iter<'a, T>
where
    T: IdReader,
    T::Id: IdOperations,
    IdRange<T::Id>: FusedIterator,
{
}

impl<'a, T> fmt::Debug for Iter<'a, T>
where
    T: IdReader + Copy,
    Self: Iterator,
    <Self as Iterator>::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}
