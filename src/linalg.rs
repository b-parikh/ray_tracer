use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
}

// Vec3 + Vec3
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ]
        }
    }
}

// Vec3 - Vec3
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ]
        }
    }
}

// Vec3 * f64
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * rhs,
                self.e[1] * rhs,
                self.e[2] * rhs,
            ]
        }
    }
}

// f64 * Vec3
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 { rhs * self }
}

// Vec3 * Vec3
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ]
        }
    }
}

// Vec3 / f64
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 { self * (1.0/rhs) }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        // In case of i < 0, function will fail before getting here as negative 
        // values aren't supported for usize types
        if i > 2 { 
            panic!("Can't access index {} of Vec3, it's out of bounds!", i);
        }

        &self.e[i]
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 { Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] } }
}

impl Vec3 {
    pub fn x(self) -> f64 { self.e[0] }
    pub fn y(self) -> f64 { self.e[1] }
    pub fn z(self) -> f64 { self.e[2] }

    fn length_squared(self) -> f64 { self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2] }
    pub fn length(self) -> f64 { self.length_squared().sqrt() }

    pub fn unit_vector(self) -> Vec3 { self / self.length() }
}


///////////////// vector utility functions /////////////////
pub mod util {
    use super::Vec3;

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 { (u[0] * v[0]) + (u[1] * v[1]) + (u[2] * v[2]) }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                (u.e[1] * v.e[2]) - (u.e[2] * v.e[1]),
                (u.e[2] * v.e[0]) - (u.e[0] * v.e[2]),
                (u.e[0] * v.e[1]) - (u.e[1] * v.e[0]),
            ]
        }
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 { *v / v.length() }
}

//////////////////////// type aliases ////////////////////////
pub type Point3 = Vec3;
pub type Color = Vec3;

//////////////////// Color util functions ////////////////////
use std::fs::File;
use std::io::Write;
impl Color {
    // write color values for a single pixel to a .ppm file
    pub fn write_color(f: &mut File, pixel_color: Color) {
        let r = (255.999 * pixel_color.x()) as u32;
        let g = (255.999 * pixel_color.y()) as u32;
        let b = (255.999 * pixel_color.z()) as u32;

        writeln!(f, "{} {} {}", r, g, b).unwrap();
    }

    pub fn test(self) {
        println!("test");
    }
}