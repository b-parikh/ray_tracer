use crate::linalg::Vec3;
use crate::linalg::Point3;


#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn origin(self) -> Point3 { self.orig }
    pub fn direction(self) -> Vec3 { self.dir }

    pub fn at(self, t: f64) -> Point3 { self.orig + t*self.dir }
}

pub mod util {
    use crate::linalg::Vec3;
    use crate::linalg::Color;
    
    use super::Ray;

    pub fn ray_color(r: &Ray, color_1: &Color, color_2: &Color) -> Vec3 {
        // Depending on the height (y) of the ray, this function linearly blends color1 and color2.
        // The blending is performed top down; that is, color_start is concentrated at the top of the image
        // and color_end is concentrated at the bottom on the image. 
        let unit_direction = r.direction().unit_vector();
        let t = 0.5*(unit_direction.y() + 1.0);

        // Linear interpolation, or "lerp"
        // If color1 is white and color2 is blue and this function is run over t: {1 -> 0}, the initial
        // colors will be blue as ~0.0 parts of white will be used. When t ~ 0, white will be seen more.
        (1.0-t) * (*color_1) + t * (*color_2)
    }
}