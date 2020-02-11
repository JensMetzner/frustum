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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_spline() {
        let p0 = Point3::<WorldSpace>::new(0.0, 0.0, 0.0);
        let p1 = Point3::<WorldSpace>::new(2.0, 1.0, 0.0);
        let p2 = Point3::<WorldSpace>::new(1.0, 2.0, 1.0);
        let p3 = Point3::<WorldSpace>::new(0.0, 2.0, 1.0);

        assert_eq!(
            catmull_rom_3d(&p0, &p1, &p2, &p3, 0.),
            Point3::<WorldSpace>::new(2.000000, 1.000000, 0.000000)
        );
        assert_eq!(
            catmull_rom_3d(&p0, &p1, &p2, &p3, 0.1),
            Point3::<WorldSpace>::new(2.021500, 1.1045000000000003, 0.064000)
        );
        assert_eq!(
            catmull_rom_3d(&p0, &p1, &p2, &p3, 0.2),
            Point3::<WorldSpace>::new(1.9919999999999998, 1.216000, 0.152000)
        );
        assert_eq!(
            catmull_rom_3d(&p0, &p1, &p2, &p3, 0.3),
            Point3::<WorldSpace>::new(1.920500, 1.331500, 0.258000)
        );
        assert_eq!(
            catmull_rom_3d(&p0, &p1, &p2, &p3, 0.4),
            Point3::<WorldSpace>::new(1.8159999999999998, 1.448000, 0.37600000000000006)
        );
        assert_eq!(
            catmull_rom_3d(&p0, &p1, &p2, &p3, 0.5),
            Point3::<WorldSpace>::new(1.687500, 1.562500, 0.500000)
        );
        assert_eq!(
            catmull_rom_3d(&p0, &p1, &p2, &p3, 0.6),
            Point3::<WorldSpace>::new(1.5440000000000003, 1.6720000000000002, 0.624000)
        );
        assert_eq!(
            catmull_rom_3d(&p0, &p1, &p2, &p3, 0.7),
            Point3::<WorldSpace>::new(1.3944999999999999, 1.773500, 0.742000)
        );
        assert_eq!(
            catmull_rom_3d(&p0, &p1, &p2, &p3, 0.8),
            Point3::<WorldSpace>::new(1.2479999999999998, 1.8639999999999999, 0.8480000000000001)
        );
        assert_eq!(
            catmull_rom_3d(&p0, &p1, &p2, &p3, 0.9),
            Point3::<WorldSpace>::new(1.1134999999999995, 1.9404999999999997, 0.9359999999999999)
        );
        assert_eq!(
            catmull_rom_3d(&p0, &p1, &p2, &p3, 1.),
            Point3::<WorldSpace>::new(1.000000, 2.000000, 1.000000)
        );
    }
}
