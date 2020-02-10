use crate::frustum::Frustum;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct FrustumPath {
    pub key_frames: Vec<Frustum>,
    pub frames_per_unit: u32,
}

impl FrustumPath {
    pub fn iter<'a>(&'a self) -> FrustumPathIterator<'a> {
        FrustumPathIterator {
            path: self,
            path_length: 0.0,
            segment_path_length: vec![0.0],
        }
    }
}

pub struct FrustumPathIterator<'a> {
    path: &'a FrustumPath,
    path_length: f64,
    segment_path_length: Vec<f64>,
}

impl<'a> Iterator for FrustumPathIterator<'a> {
    type Item = Frustum;

    fn next(&mut self) -> Option<Self::Item> {
        if self.path_length > 0.0 {
            None
        } else {
            self.path_length += 1.0;
            Some(self.path.key_frames[0])
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(5))
    }
}
