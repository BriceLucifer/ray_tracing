use crate::{
    color::{Color, write_color},
    hittable::{HitRecord, HitTable},
    hittable_list::HitTableList,
    interval::Interval,
    ray::{Point3, Ray},
    rtweekend::INFINITY,
    vec3::Vec3,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Point3,
    pixel_delta_v: Point3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 1.0;
        let image_width = 100;
        let image_height: u32 = if ((image_width as f64 / aspect_ratio) as u32) < 1 {
            1 as u32
        } else {
            (image_width as f64 / aspect_ratio) as u32
        };

        let center = Point3::new();

        // determine the viewport dimensions
        let focal_length = 1.0;
        let viewpoint_height = 2.0;
        let viewpoint_width = viewpoint_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewpoint edges.

        let viewport_u = Vec3::new_with_value(viewpoint_width, 0.0, 0.0);
        let viewport_v = Vec3::new_with_value(0.0, -viewpoint_height, 0.0);

        let pixel_delta_u = viewport_u.clone() / image_width as f64;
        let pixel_delta_v = viewport_v.clone() / image_height as f64;
        let viewport_upper_left = center.clone()
            - Vec3::new_with_value(0.0, 0.0, focal_length)
            - viewport_u.clone() / 2.0
            - viewport_v.clone() / 2.0;
        let pixel00_loc =
            viewport_upper_left + 0.5 * (pixel_delta_u.clone() + pixel_delta_v.clone());

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn ray_color(&self, r: &Ray, world: &Box<HitTableList>) -> Color {
        let mut rec = HitRecord::new();

        if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new_with_value(1.0, 1.0, 1.0));
        }

        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new_with_value(1.0, 1.0, 1.0)
            + a * Color::new_with_value(0.5, 0.7, 1.0);
    }

    pub fn init(&mut self) {
        self.image_height = if ((self.image_width as f64 / self.aspect_ratio) as u32) < 1 {
            1 as u32
        } else {
            (self.image_width as f64 / self.aspect_ratio) as u32
        };

        self.center = Point3::new();

        // determine the viewport dimensions
        let focal_length = 1.0;
        let viewpoint_height = 2.0;
        let viewpoint_width = viewpoint_height * self.aspect_ratio;

        // Calculate the vectors across the horizontal and down the vertical viewpoint edges.

        let viewport_u = Vec3::new_with_value(viewpoint_width, 0.0, 0.0);
        let viewport_v = Vec3::new_with_value(0.0, -viewpoint_height, 0.0);

        self.pixel_delta_u = viewport_u.clone() / self.image_width as f64;
        self.pixel_delta_v = viewport_v.clone() / self.image_height as f64;
        let viewport_upper_left = self.center.clone()
            - Vec3::new_with_value(0.0, 0.0, focal_length)
            - viewport_u.clone() / 2.0
            - viewport_v.clone() / 2.0;
        self.pixel00_loc =
            viewport_upper_left + 0.5 * (self.pixel_delta_u.clone() + self.pixel_delta_v.clone());
    }

    pub fn render(&mut self, world: &Box<HitTableList>) {
        self.init();
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");
        for j in 0..self.image_height {
            print!("\rScanlines remainning: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc.clone()
                    + (i as f64 * self.pixel_delta_u.clone())
                    + (j as f64 * self.pixel_delta_v.clone());
                let ray_direction = pixel_center - self.center.clone();
                let r = Ray::new(self.center.clone(), ray_direction);
                let pixel_color = self.ray_color(&r, world);
                write_color(&pixel_color);
            }
        }
        eprintln!("Render complete");
    }
}
