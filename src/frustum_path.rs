use crate::frustum::Frustum;
// use crate::spline::catmull_rom_3d;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Default, Clone)]
pub struct FrustumPath {
    pub key_frustums: Vec<Frustum>,
    pub frames_per_unit: u32,
    #[cfg_attr(feature = "serialization", serde(skip))]
    interpolated_frustums: Option<Vec<Frustum>>,
}

// impl FrustumPath {
//     fn path_length(&self) -> f64 {
//     }

//     fn get_control_points_for_segment(&self, segment_id: usize) -> (usize, usize, usize, usize) {
//         let cp0 = segment_id.min(self.key_frustums.len() - 2) - 1;

//         (
//             cp0.max(0),
//             cp0 + 1,
//             cp0 + 2,
//             (cp0 + 3).min(self.key_frustums.len() - 1),
//         )
//     }

//     fn get_length_for_segment(&self, segment_id: usize) -> f64 {
//         let (cp0, cp1, cp2, cp3) = self.get_control_points_for_segment(segment_id);

//         let mut last = self.key_frustums[segment_id].origin;
//         if (last - self.key_frustums[segment_id + 1].origin).length() < std::f64::EPSILON {
//             return 0.;
//         }
//         let mut length = 0.0;
//         for i in 0..10000 {
//             let t = i as f64 * 0.0001;
//             let current = catmull_rom_3d(
//                 &self.key_frustums[cp0].origin,
//                 &self.key_frustums[cp1].origin,
//                 &self.key_frustums[cp2].origin,
//                 &self.key_frustums[cp3].origin,
//                 t,
//             );
//             length += (current - last).length();
//             last = current;
//         }
//         length
//     }
// }

// impl FrustumPath {
//     pub fn iter<'a>(&'a self) -> FrustumPathIterator<'a> {
//         FrustumPathIterator {
//             frustum_path: self,
//             current_idx: 0,
//         }
//     }
// }

// pub struct FrustumPathIterator<'a> {
//     frustum_path: &'a FrustumPath,
//     current_idx: u32,
// }

// impl<'a> Iterator for FrustumPathIterator<'a> {
//     type Item = Frustum;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.path_length > 0.0 {
//             None
//         } else {
//             self.path_length += 1.0;
//             Some(self.path.key_frustums[0])
//         }
//     }

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         (0, Some(5))
//     }
// }
