use crate::frustum::{Frustum, FrustumIterator};
use crate::types::*;
use rayon::iter::plumbing::{
    bridge_producer_consumer, Consumer, Folder, Producer, ProducerCallback, UnindexedConsumer,
    UnindexedProducer,
};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

pub struct FrustumParallelIterator<'a> {
    frustum: &'a Frustum,
    position: [usize; 2],
}

impl<'a> Producer for FrustumParallelIterator<'a> {
    type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);
    type IntoIter = FrustumIterator<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        FrustumIterator {
            frustum: self.frustum,
            position: self.position,
            is_done: false,
        }
    }

    #[inline]
    fn split_at(self, index: usize) -> (Self, Self) {
        (
            FrustumParallelIterator {
                frustum: self.frustum,
                position: [self.position[0], self.position[0] + index - 1],
            },
            FrustumParallelIterator {
                frustum: self.frustum,
                position: [self.position[0] + index, self.position[1]],
            },
        )
    }
}

impl<'a> ParallelIterator for FrustumParallelIterator<'a> {
    type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);

    #[inline]
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge_producer_consumer(self.frustum.width * self.frustum.height, self, consumer)
    }
}

impl<'a> IntoParallelIterator for &'a Frustum {
    type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);
    type Iter = FrustumParallelIterator<'a>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        FrustumParallelIterator {
            frustum: self,
            position: [0, (self.width * self.height) - 1],
        }
    }
}

impl<'a> IntoParallelRefIterator<'a> for &'a Frustum {
    type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);
    type Iter = FrustumParallelIterator<'a>;

    #[inline]
    fn par_iter(&'a self) -> Self::Iter {
        self.into_par_iter()
    }
}

// impl<'a> UnindexedProducer for FrustumParallelIterator<'a> {
//     type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);

//     #[inline]
//     fn split(self) -> (Self, Option<Self>) {
//         // bridge_unindexed(self, consumer)
//     }

//     #[inline]
//     fn fold_with<F>(self, folder: F) -> F
//     where
//         F: Folder<Self::Item>,
//     {
//     }
// }

// impl<'a> IndexedParallelIterator for FrustumParallelIterator<'a> {
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
