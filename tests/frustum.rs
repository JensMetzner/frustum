#[cfg(test)]
mod tests {
    use frustum::*;

    fn default_camera() -> Frustum {
        Frustum {
            origin: Point3::<WorldSpace>::new(0.0, 0.0, 10.0),
            target: Point3::<WorldSpace>::new(0.0, 0.0, 0.0),
            fovy: 45.0,
            ncp: 1.0,
            fcp: 20.0,
            width: 500,
            height: 500,
        }
    }

    #[test]
    fn test_view() {
        assert_eq!(
            default_camera().view(),
            Transform3::row_major(
                -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, -0.0, -0.0, 10.0, 1.0
            )
        )
    }

    #[test]
    fn test_projection() {
        assert_eq!(
            default_camera().projection(),
            Transform3::row_major(
                2.414213562373095,
                0.0,
                0.0,
                0.0,
                0.0,
                -2.414213562373095,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0526315789473684,
                1.0,
                0.0,
                0.0,
                -1.0526315789473684,
                0.0
            )
        )
    }

    #[test]
    fn test_screen() {
        assert_eq!(
            default_camera().screen(),
            Transform3::column_major(
                250.0, 0.0, 0.0, 250.0, 0.0, 250.0, 0.0, 250.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                1.0
            )
        )
    }

    #[test]
    fn test_proj_view() {
        let camera = default_camera();
        let view = camera.view();
        let projection = camera.projection();

        assert_eq!(
            view.post_transform(&projection),
            Transform3::row_major(
                -2.414213562373095,
                0.0,
                0.0,
                0.0,
                0.0,
                -2.414213562373095,
                0.0,
                0.0,
                0.0,
                0.0,
                -1.0526315789473684,
                -1.0,
                0.0,
                0.0,
                9.473684210526315,
                10.0
            )
        )
    }

    #[test]
    fn test_screen_proj_view() {
        let camera = default_camera();
        let view = camera.view();
        let projection = camera.projection();
        let screen = camera.screen();

        assert_eq!(
            view.post_transform(&projection)
                .post_transform(&screen)
                .transform_point3d(Point3::new(0.0, 0.0, -11.0)),
            Some(Point3::new(250.0, 250.0, 1.0025062656641603))
        );
        assert_eq!(
            view.post_transform(&projection)
                .post_transform(&screen)
                .transform_point3d(Point3::new(0.0, 0.0, 9.0)),
            Some(Point3::new(250.0, 250.0, 0.0))
        );
    }
}
