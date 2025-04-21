pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ppm;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;

#[macro_export]
macro_rules! dot {
    ($v1:expr, $v2:expr) => {
        Vec3::dot($v1, $v2)
    };
}

#[macro_export]
macro_rules! fmax {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}
