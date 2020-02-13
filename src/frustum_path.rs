use crate::frustum::Frustum;
use crate::spline::{Spline, Spline3};

#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Default, Clone)]
pub struct FrustumPath {
    pub key_frustums: Vec<Frustum>,
    pub frames_per_unit: u32,
}

impl FrustumPath {
    pub fn iter<'a>(&'a self) -> FrustumPathIterator<'a> {
        let origins = self
            .key_frustums
            .iter()
            .map(|frustum| frustum.origin)
            .collect::<Vec<_>>();
        let targets = self
            .key_frustums
            .iter()
            .map(|frustum| frustum.target)
            .collect::<Vec<_>>();

        let (_, origin_segments_length) = Spline3::length(&origins);
        let (_, target_segments_length) = Spline3::length(&targets);

        let longer_segments = origin_segments_length
            .iter()
            .zip(target_segments_length.iter())
            .map(|(o, t)| {
                if o >= t {
                    LongerSegment::Origin(*o)
                } else {
                    LongerSegment::Target(*t)
                }
            })
            .collect::<Vec<_>>();

        let path_length = longer_segments.iter().map(|s| s.value()).sum();

        FrustumPathIterator {
            frustum_path: self,
            path_length,
            position_on_path: 0.,
            longer_segments,
            current_segment: 0,
        }
    }
}

enum LongerSegment {
    Origin(f64),
    Target(f64),
}

impl LongerSegment {
    fn value(&self) -> &f64 {
        match self {
            Self::Origin(v) => v,
            Self::Target(v) => v,
        }
    }
}

pub struct FrustumPathIterator<'a> {
    frustum_path: &'a FrustumPath,
    path_length: f64,
    position_on_path: f64,
    longer_segments: Vec<LongerSegment>,
    current_segment: usize,
}

impl<'a> FrustumPathIterator<'a> {}

impl<'a> Iterator for FrustumPathIterator<'a> {
    type Item = Frustum;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position_on_path < 0.
            || self.position_on_path > self.path_length
        {
            return None;
        }

        // TODO

        // self.path_length += 1.0;
        // Some(self.path.key_frustums[0])
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(5))
    }
}
