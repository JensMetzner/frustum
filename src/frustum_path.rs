use crate::frustum::Frustum;
use crate::spline::{Spline, Spline1, Spline3};

#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Default, Clone)]
pub struct FrustumPath {
    pub key_frustums: Vec<Frustum>,
    pub frames_per_unit: u8,
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

        let (_, origin_segment_lengths) = Spline3::length(&origins);
        let (_, target_segment_lengths) = Spline3::length(&targets);

        let max_segment_lengths = origin_segment_lengths
            .iter()
            .zip(target_segment_lengths.iter())
            .map(|(o, t)| {
                if o >= t {
                    LongerSegment::Origin(*o)
                } else {
                    LongerSegment::Target(*t)
                }
            })
            .collect::<Vec<_>>();

        let path_length =
            max_segment_lengths.iter().map(|s| s.as_value()).sum();

        FrustumPathIterator {
            frustum_path: self,
            path_length,
            length_per_frame: 1. / self.frames_per_unit as f64,
            position_on_path: 0.,
            max_segment_lengths,
        }
    }
}

enum LongerSegment {
    Origin(f64),
    Target(f64),
}

impl LongerSegment {
    fn as_value(&self) -> &f64 {
        match self {
            Self::Origin(v) => v,
            Self::Target(v) => v,
        }
    }
}

pub struct FrustumPathIterator<'a> {
    frustum_path: &'a FrustumPath,
    path_length: f64,
    length_per_frame: f64,
    max_segment_lengths: Vec<LongerSegment>,
    position_on_path: f64,
}

impl<'a> FrustumPathIterator<'a> {}

impl<'a> Iterator for FrustumPathIterator<'a> {
    type Item = Frustum;

    fn next(&mut self) -> Option<Self::Item> {
        let mut segment_idx = self.position_on_path as usize;
        let mut position_on_segment = self.position_on_path.fract();

        let mut remaining = self.length_per_frame;

        for segment_length in self.max_segment_lengths[segment_idx..].iter() {
            let remaining_on_segment = segment_length.as_value()
                - segment_length.as_value() * position_on_segment;

            if remaining_on_segment >= remaining {
                position_on_segment += remaining / segment_length.as_value();
                break;
            } else {
                remaining -= remaining_on_segment;
                segment_idx += 1;
                position_on_segment = 0.;
            }
        }

        self.position_on_path = segment_idx as f64 + position_on_segment;

        if segment_idx > self.max_segment_lengths.len() - 1 {
            return None;
        }

        let (idx0, idx1, idx2, idx3) = Spline3::get_control_points_for_segment(
            segment_idx,
            self.frustum_path.key_frustums.len(),
        );

        let origin = Spline3::catmull_rom(
            &self.frustum_path.key_frustums[idx0].origin,
            &self.frustum_path.key_frustums[idx1].origin,
            &self.frustum_path.key_frustums[idx2].origin,
            &self.frustum_path.key_frustums[idx3].origin,
            position_on_segment,
        );

        let target = Spline3::catmull_rom(
            &self.frustum_path.key_frustums[idx0].target,
            &self.frustum_path.key_frustums[idx1].target,
            &self.frustum_path.key_frustums[idx2].target,
            &self.frustum_path.key_frustums[idx3].target,
            position_on_segment,
        );

        let fovy = Spline1::catmull_rom(
            &self.frustum_path.key_frustums[idx0].fovy,
            &self.frustum_path.key_frustums[idx1].fovy,
            &self.frustum_path.key_frustums[idx2].fovy,
            &self.frustum_path.key_frustums[idx3].fovy,
            position_on_segment,
        );

        let ncp = Spline1::catmull_rom(
            &self.frustum_path.key_frustums[idx0].ncp,
            &self.frustum_path.key_frustums[idx1].ncp,
            &self.frustum_path.key_frustums[idx2].ncp,
            &self.frustum_path.key_frustums[idx3].ncp,
            position_on_segment,
        );

        let fcp = Spline1::catmull_rom(
            &self.frustum_path.key_frustums[idx0].fcp,
            &self.frustum_path.key_frustums[idx1].fcp,
            &self.frustum_path.key_frustums[idx2].fcp,
            &self.frustum_path.key_frustums[idx3].fcp,
            position_on_segment,
        );

        Some(Frustum {
            origin,
            target,
            fovy,
            ncp,
            fcp,
            width: self.frustum_path.key_frustums[0].width,
            height: self.frustum_path.key_frustums[0].height,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            (self.path_length / self.length_per_frame) as usize,
            Some((self.path_length / self.length_per_frame).ceil() as usize),
        )
    }
}
