use vec3::{Vec3, Vec3F32};

pub struct Ray {
    pub orig: Vec3<f32>,
    pub dir: Vec3<f32>,
}

impl Ray {
    pub fn with(orig: Vec3<f32>, dir: Vec3<f32>) -> Self {
        Self {
            orig,
            dir,
        }
    }
    #[allow(dead_code)]
    pub fn at(&self, t: f32) -> Vec3<f32> {
        let temp: Vec3<f32> = self.dir.mul(t);
        self.orig.add(temp)
    }
}