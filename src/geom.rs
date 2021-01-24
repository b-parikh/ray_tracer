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

    // A ray is defined with a linear equation: 
    // P(t) = A + tb
    // A = origin; b = direction; P(t) = point on ray at time = t
    pub fn at(self, t: f64) -> Point3 { self.orig + t*self.dir }
}

pub mod util {
    use crate::linalg:: {Vec3, Point3, Color};
    use crate::linalg::util:: {dot, unit_vector};

    use super::Ray;

    pub fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
        /*
        (x - Cx)^2 + (y - Cy)^2 + (z - Cz)^2 = r^2 means (x,y,z) is on the sphere
                                             > r^2 means (x,y,z) is outside the sphere
                                             < r^2 means (x,y,z) is inside the sphere
        Putting this in vector notation means that (P-C)*(P-C) = r^2 yields all points P
        that are on the sphere. Since P(t) = A + tb (definition of a ray), plugging this in and
        doing some rearranging yields a quadratic equation: t^2*dot(b,b) + 2t*dot(b,(A-C)) + dot((A-C),(A-C)) - r^2 = 0.
        If this equation has two roots, the ray with origin A and direction b will go through the sphere.
        One root means it is tangent to the sphere. No roots means it misses the sphere. We can check
        this by calculating the discriminant D. 
        D > 0 -> two roots
        D == 0 -> one root
        D < 0 -> no roots
        If the ray missed, return -1.0. If it hit, return the root where t > 0.
        No need to worry about the t < 0 root since the sphere is currently only in front of the camera.
        */
        let oc = r.origin() - center; // (A-C)
        let a = r.direction().length_squared(); // dot(b, b); vector dotted with itself is the sq. length of itself
        let half_b = dot(&oc, &r.direction()); // 2*dot(b,(A-C)); see 6.12 in book for explanation of what half_b is
        let c = oc.length_squared() - radius*radius; // dot((A-C), (A-C)) - r^2
        let discriminant = half_b*half_b - a*c;
        
        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / (a)
        }
    }

    pub fn ray_color(r: &Ray, color_1: &Color, color_2: &Color) -> Vec3 {
        let t = hit_sphere(Point3 { e: [0.0, 0.0, -1.0] }, 0.5, *r);
        // If sphere at (0, 0, -1) with radius = 5 is hit, return the sphere's color (red) instead of drawing the background.
        // if t > 0, then we know the ray hit the sphere at t. Get the unit normal vector at the point of intersection and color it
        // based on its position in 3D space.
        if t > 0.0 {
            let N = unit_vector(&(r.at(t) - Vec3 { e: [0.0, 0.0, -1.0] }));
            return 0.5 * Color { e: [N.x() + 1.0, N.y() + 1.0,  N.z() + 1.0] };
        }

        // Depending on the height (y) of the ray, this function linearly blends color1 and color2.
        // The blending is performed top down; that is, color_start is concentrated at the top of the image
        // and color_end is concentrated at the bottom on the image. 
        let unit_direction = r.direction().unit_vector(); // ray didn't it; it's direction continues to be the same
        let t = 0.5*(unit_direction.y() + 1.0);

        // Linear interpolation, or "lerp"
        // If color1 is white and color2 is blue and this function is run over t: {1 -> 0}, the initial
        // colors will be blue as ~0.0 parts of white will be used. When t ~ 0, white will be seen more.
        (1.0-t) * (*color_1) + t * (*color_2)
    }
}