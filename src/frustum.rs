use crate::types::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const UP: Vec3<WorldSpace> = Vec3::<WorldSpace>::new(0.0, 1.0, 0.0);

/// Frustum struct
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
            x: 0,
            y: 0,
        }
    }
}

pub struct FrustumIterator<'a> {
    frustum: &'a Frustum,
    x: usize,
    y: usize,
}

impl<'a> Iterator for FrustumIterator<'a> {
    type Item = (Point3<WorldSpace>, Vec3<WorldSpace>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.frustum.width {
            self.x = 0;
            self.y += 1;
        }

        if self.y >= self.frustum.height {
            None
        } else {
            let t = self
                .frustum
                .ray_from_ncp(&Point2::<ScreenSpace>::new(self.x as f64, self.y as f64));
            self.x += 1;
            t
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.frustum.width * self.frustum.height) as usize;
        let done = self.x + self.y * self.frustum.width as usize;
        (size - done, Some(size - done))
    }
}
