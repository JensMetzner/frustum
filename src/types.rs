use euclid::*;

pub struct WorldSpace;
pub struct CameraSpace;
pub struct ViewSpace;
pub struct ScreenSpace;

pub type Vec2<T> = Vector2D<f64, T>;
pub type Vec3<T> = Vector3D<f64, T>;

pub type Point2<T> = Point2D<f64, T>;
pub type Point3<T> = Point3D<f64, T>;

pub type Rotation2<S, T> = Rotation2D<f64, S, T>;
pub type Rotation3<S, T> = Rotation3D<f64, S, T>;

pub type Transform2<S, T> = Transform2D<f64, S, T>;
pub type Transform3<S, T> = Transform3D<f64, S, T>;
