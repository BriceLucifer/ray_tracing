use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (255.99 * r) as i32;
    let gbyte = (255.99 * g) as i32;
    let bbyte = (255.99 * b) as i32;

    print!("{} {} {}\n", rbyte, gbyte, bbyte);
}
