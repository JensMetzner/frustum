use crate::types::*;

pub fn catmull_rom_3d(
    v1: &Point3<WorldSpace>,
    v2: &Point3<WorldSpace>,
    v3: &Point3<WorldSpace>,
    v4: &Point3<WorldSpace>,
    s: f64,
) -> Point3<WorldSpace> {
    let s2 = s * s;
    let s3 = s2 * s;

    let f1 = -s3 + 2. * s2 - s;
    let f2 = 3. * s3 - 5. * s2 + 2.;
    let f3 = -3. * s3 + 4. * s2 + s;
    let f4 = s3 - s2;

    (*v1 * f1 + v2.to_vector() * f2 + v3.to_vector() * f3 + v4.to_vector() * f4) * 0.5
}

#[cfg(test)]
mod tests {
    use crate::spline::catmull_rom_3d;
    use crate::types::*;
    #[test]
    fn test_spline() {
        let p0 = Point3::<WorldSpace>::new(0.0, 0.0, 0.0);
        let p1 = Point3::<WorldSpace>::new(2.0, 1.0, 0.0);
        let p2 = Point3::<WorldSpace>::new(1.0, 2.0, 1.0);
        let p3 = Point3::<WorldSpace>::new(0.0, 2.0, 1.0);

        assert_eq!(
            catmull_rom_3d(&p0, &p1, &p2, &p3, 0.5),
            Point3::<WorldSpace>::new(1.6875, 1.5625, 0.5)
        )
    }
}
