use frustum::{Frustum, Point3, Vec3, WorldSpace};
use palette::*;
use rayon::prelude::*;

pub struct Sphere {
    origin: Point3<WorldSpace>,
    radius: f64,
    ambient: palette::Lab,
    diffuse: palette::Lab,
    specular: palette::Lab,
}

impl Sphere {
    pub fn intersect(
        &self,
        ro: &Point3<WorldSpace>,
        rd: &Vec3<WorldSpace>,
    ) -> Option<(Point3<WorldSpace>, f64)> {
        let diff = self.origin - *ro;
        let t0 = diff.dot(*rd);
        let d_squared = diff.dot(diff) - t0 * t0;
        if d_squared > (self.radius * self.radius) {
            return None;
        }

        let t1 = (self.radius * self.radius) - d_squared;

        let t = if t0 > t1 + std::f64::EPSILON {
            t0 - t1
        } else {
            t0 + t1
        };

        if t > std::f64::EPSILON {
            return Some((*ro + *rd * t, t));
        }
        None
    }

    pub fn normal(&self, target: &Point3<WorldSpace>) -> Vec3<WorldSpace> {
        (*target - self.origin).normalize()
    }

    pub fn color(
        &self,
        target: &Point3<WorldSpace>,
        light: &Point3<WorldSpace>,
        origin: &Point3<WorldSpace>,
    ) -> palette::Lab {
        let normal = self.normal(target).normalize();
        let light_dir = (*light - *target).normalize();
        let view_dir = (*origin - *target).normalize();
        let half_dir = (light_dir + view_dir).normalize();

        let lambertian = normal.dot(light_dir).max(0.).min(1.) as f32;
        let specular = if lambertian > 0.0 {
            half_dir.dot(normal).max(0.).powi(128).min(1.) as f32
        } else {
            0.0
        };

        self.ambient
            .mix(&self.diffuse, lambertian * 0.7)
            .mix(&self.specular, specular * 1.5)
            .clamp()
    }
}

fn main() {
    let light = Point3::<WorldSpace>::new(-5., 5., 20.);

    let sphere = Sphere {
        origin: Point3::<WorldSpace>::new(0., 0., 0.),
        radius: 2.5,
        ambient: palette::Lab::new(20., -6., -11.),
        diffuse: palette::Lab::new(86., -9., 14.),
        specular: palette::Lab::new(100., -3., 4.),
    };

    let camera = Frustum {
        origin: Point3::<WorldSpace>::new(0.0, 0.0, 10.0),
        target: Point3::<WorldSpace>::new(0.0, 0.0, 0.0),
        fovy: 45.0,
        ncp: 1.0,
        fcp: 20.0,
        width: 500,
        height: 500,
    };

    let data = camera
        .par_iter()
        .map(|(ro, rd)| {
            let color: Srgb = match sphere.intersect(&ro, &rd) {
                Some((target, _)) => sphere.color(&target, &light, &camera.origin),
                None => palette::Lab::new(100., 0., 0.),
            }
            .into();

            color.clamp().into_raw::<[f32; 3]>()
        })
        .collect::<Vec<[f32; 3]>>()
        .iter()
        .flatten()
        .map(|v| (*v * 255.0) as u8)
        .collect::<Vec<u8>>();

    image::save_buffer(
        "raytracing_result.png",
        &data,
        camera.width as u32,
        camera.height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap_or_else(|err| {
        println!("Cannot generate photorealistic image: {}", err);
        std::process::exit(4);
    });
}
