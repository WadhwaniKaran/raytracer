#[derive(Clone)]
pub struct Vec3<T> {
    pub e: [T; 3],
}

pub trait Vec3F32 {
    fn new() -> Self;
    fn with(x: f32, y: f32, z: f32) -> Self;
    fn add(&self, other: Vec3<f32>) -> Vec3<f32>;
    fn sub(&self, other: Vec3<f32>) -> Vec3<f32>;
    fn mul(&self, other: f32) -> Vec3<f32>;
    fn div(&self, other: f32) -> Vec3<f32>;
    fn dot_product(&self, other: Vec3<f32>) -> f32;
    fn cross_product(&self, other: Vec3<f32>) -> Vec3<f32>;
    fn length(&self) -> f32;
    fn unit_vec(&self) -> Vec3<f32>;
}


impl Copy for Vec3<f32> {}
impl Vec3F32 for Vec3<f32> {
    fn new() -> Self {
        Self {
            e: [1_f32, 1_f32, 1_f32],
        }
    }

    fn with(x: f32, y: f32, z: f32) -> Self {
        Self {
            e: [x, y, z],
        }
    }

    fn add(&self, other: Vec3<f32>) -> Vec3<f32> {
        let x = self.e[0] + other.e[0];
        let y = self.e[1] + other.e[1];
        let z = self.e[2] + other.e[2];
        Vec3::with(x, y, z)
    }

    fn sub(&self, other: Vec3<f32>) -> Vec3<f32>{
        let x = self.e[0] - other.e[0];
        let y = self.e[1] - other.e[1];
        let z = self.e[2] - other.e[2];
        Vec3::with(x, y, z)
    }

    fn mul(&self, other: f32) -> Vec3<f32>{
        let x = self.e[0] * other;
        let y = self.e[1] * other;
        let z = self.e[2] * other;
        Vec3::with(x, y, z)
    }

    fn div(&self, other: f32) -> Vec3<f32>{
        let x = self.e[0] / other;
        let y = self.e[1] / other;
        let z = self.e[2] / other;
        Vec3::with(x, y, z)
    }

    fn dot_product(&self, other: Vec3<f32>) -> f32 {
        let x = self.e[0] * other.e[0];
        let y = self.e[1] * other.e[1];
        let z = self.e[2] * other.e[2];
        x + y + z
    }

    fn cross_product(&self, other: Vec3<f32>)-> Vec3<f32> {
        let x = self.e[1] * other.e[2] - self.e[2] * other.e[1];
        let y: f32 = self.e[0] * other.e[2] - self.e[2] * other.e[0];
        let z = self.e[0] * other.e[1] - self.e[1] * other.e[0];
        Vec3::with(x, -y, z)
    }

    fn length(&self) -> f32 {
        let mag = f32::powf(
            f32::powf(self.e[0], 2.0) + 
            f32::powf(self.e[1], 2.0) + 
            f32::powf(self.e[2], 2.0), 
            0.5);
        mag
    }

    fn unit_vec(&self) -> Vec3<f32> {
        let mag = self.length();
        Vec3::with(self.e[0] / mag, self.e[1] / mag, self.e[2] / mag)
    }
}