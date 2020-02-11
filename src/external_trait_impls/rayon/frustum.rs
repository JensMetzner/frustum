// use crate::frustum::Frustum;
// use crate::types::*;
// use rayon::iter::plumbing::{bridge, Consumer, Producer, UnindexedConsumer, ProducerCallback};
// use rayon::iter::{
//     IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
// };

// pub struct ParIter<'a> {
//     frustum: &'a Frustum,
// }

// impl<'a> IndexedParallelIterator for ParIter<'a> {
//     type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);

//     #[inline]
//     fn drive<C>(self, consumer: C) -> C::Result
//     where
//         C: Consumer<Self::Item>,
//     {
//         bridge(self, consumer)
//     }

//     #[inline]
//     fn len(&self) -> usize {
//         (self.frustum.width * self.frustum.height) as usize
//     }

//     #[inline]
//     fn with_producer<CB>(self, callback: CB) -> CB::Output
//     where
//         CB: ProducerCallback<Self::Item>,
//     {
//     }
// }

// impl<'a> ParallelIterator for ParIter<'a> {
//     type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);

//     #[inline]
//     fn drive_unindexed<C>(self, consumer: C) -> C::Result
//     where
//         C: UnindexedConsumer<Self::Item>,
//     {
//         bridge(self, consumer)
//     }
// }

// pub struct IntoParIter {
//     frustum: Frustum,
// }

// impl ParallelIterator for IntoParIter {
//     type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);

//     #[inline]
//     fn drive_unindexed<C>(self, consumer: C) -> C::Result
//     where
//         C: UnindexedConsumer<Self::Item>,
//     {
//         self.frustum.par_iter().drive_unindexed(consumer)
//     }
// }

// impl IntoParallelIterator for Frustum {
//     type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);
//     type Iter = IntoParIter;

//     #[inline]
//     fn into_par_iter(self) -> Self::Iter {
//         IntoParIter { frustum: self }
//     }
// }

// impl<'a> IntoParallelIterator for &'a Frustum {
//     type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);
//     type Iter = ParIter<'a>;

//     #[inline]
//     fn into_par_iter(self) -> Self::Iter {
//         ParIter { frustum: self }
//     }
// }

// impl<'a> IntoParallelRefIterator<'a> for &'a Frustum {
//     type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);
//     type Iter = ParIter<'a>;

//     #[inline]
//     fn par_iter(&'a self) -> Self::Iter {
//         self.into_par_iter()
//     }
// }
