use crate::types::*;

const UP: Vec3<WorldSpace> = Vec3::<WorldSpace>::new(0.0, 1.0, 0.0);

/// Frustum struct
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Debug, Copy, Clone)]
pub struct Frustum {
    pub origin: Point3<WorldSpace>,
    pub target: Point3<WorldSpace>,
    pub fovy: f64,
    pub ncp: f64,
    pub fcp: f64,
    pub width: usize,
    pub height: usize,
}

impl Frustum {
    /// Generates the view matrix.
    /// Using left hand.
    pub fn view(&self) -> Transform3<WorldSpace, CameraSpace> {
        let f = (self.target - self.origin).normalize();
        let s = UP.cross(f).normalize();
        let u = f.cross(s);

        Transform3::column_major(
            s.x,
            s.y,
            s.z,
            -(s.dot(self.origin.to_vector())),
            u.x,
            u.y,
            u.z,
            -(u.dot(self.origin.to_vector())),
            f.x,
            f.y,
            f.z,
            -(f.dot(self.origin.to_vector())),
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    /// Generates the perspective projection matrix.
    /// Using left hand with zero to one (y flip).
    pub fn projection(&self) -> Transform3<CameraSpace, ViewSpace> {
        let tan_half_fovy = (self.fovy.to_radians() / 2.0).tan();
        let aspect = self.width as f64 / self.height as f64;

        Transform3::column_major(
            1.0 / (aspect * tan_half_fovy),
            0.0,
            0.0,
            0.0,
            0.0,
            -1.0 / tan_half_fovy,
            0.0,
            0.0,
            0.0,
            0.0,
            self.fcp / (self.fcp - self.ncp),
            -(self.fcp * self.ncp) / (self.fcp - self.ncp),
            0.0,
            0.0,
            1.0,
            0.0,
        )
    }

    /// Generates the screen matrix.
    pub fn screen(&self) -> Transform3<ViewSpace, ScreenSpace> {
        Transform3::column_major(
            self.width as f64 / 2.0,
            0.0,
            0.0,
            self.width as f64 / 2.0,
            0.0,
            self.height as f64 / 2.0,
            0.0,
            self.height as f64 / 2.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    /// Calculate for a given screen space coordinate the corresponding ray origin and direction on the near clipping plane.
    pub fn ray_from_ncp(
        &self,
        screen_coords: &Point2<ScreenSpace>,
    ) -> Option<(Point3<WorldSpace>, Vec3<WorldSpace>)> {
        let view = self.view().inverse().expect("Inversing view failed.");
        let projection = self
            .projection()
            .inverse()
            .expect("Inversing projection failed.");
        let screen = self.screen().inverse().expect("Inversing screen failed.");

        let ro = screen
            .post_transform(&projection)
            .post_transform(&view)
            .transform_point3d(screen_coords.to_3d())?;

        let rd = (ro - self.origin).normalize();

        Some((ro, rd))
    }

    pub fn distance(&self, position: &Point3<WorldSpace>) -> f64 {
        (self.origin - *position).length() - self.ncp
    }

    pub fn iter(&self) -> FrustumIterator {
        FrustumIterator {
            frustum: self,
            position: [0, (self.width * self.height) - 1],
            is_done: false,
        }
    }
}

pub struct FrustumIterator<'a> {
    pub frustum: &'a Frustum,
    pub position: [usize; 2],
    pub is_done: bool,
}

impl<'a> Iterator for FrustumIterator<'a> {
    type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);
    // type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let x = self.position[0] % self.frustum.width;
        let y = self.position[0] / self.frustum.width;

        let t = self
            .frustum
            .ray_from_ncp(&Point2::<ScreenSpace>::new(x as f64, y as f64));
        // let t = Some((x, y));

        if self.position[0] >= self.position[1] {
            self.is_done = true;
        } else {
            self.position[0] += 1;
        }

        t
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a> ExactSizeIterator for FrustumIterator<'a> {
    fn len(&self) -> usize {
        self.position[1] - self.position[0]
    }
}

impl<'a> DoubleEndedIterator for FrustumIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let x = self.position[1] as usize % self.frustum.width;
        let y = self.position[1] as usize / self.frustum.width;

        let t = self
            .frustum
            .ray_from_ncp(&Point2::<ScreenSpace>::new(x as f64, y as f64));
        // let t = Some((x, y));

        if self.position[0] >= self.position[1] {
            self.is_done = true;
        } else {
            self.position[1] -= 1;
        }
        t
    }
}

// #[test]
// fn name() {
//     let camera = Frustum {
//         origin: Point3::<WorldSpace>::new(0.0, 0.0, 10.0),
//         target: Point3::<WorldSpace>::new(0.0, 0.0, 0.0),
//         fovy: 45.0,
//         ncp: 1.0,
//         fcp: 20.0,
//         width: 5,
//         height: 5,
//     };

//     let mut it = camera.iter();

//     for y in 0..=2usize {
//         for x in 0..=4usize {
//             assert_eq!(dbg!(it.next()), dbg!(Some((x, y))));
//         }
//     }
//     for y in (3..=4usize).rev() {
//         for x in (0..=4usize).rev() {
//             assert_eq!(dbg!(it.next_back()), dbg!(Some((x, y))));
//         }
//     }

//     assert_eq!(it.next(), None)
// }
