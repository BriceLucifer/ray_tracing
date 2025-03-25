pub mod color;
pub mod ppm;
pub mod ray;
pub mod vec3;

#[macro_export]
macro_rules! dot {
    ($v1:expr, $v2:expr) => {
        Vec3::dot($v1, $v2)
    };
}
