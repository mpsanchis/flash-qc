use crate::math::{Mat4, Quaternion, Vec3};

pub struct Camera {
    pub distance: f32,
    pub rotation: Quaternion,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            distance: 8.0,
            rotation: Quaternion::identity(),
            fov: 45.0_f32.to_radians(),
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        let rot_matrix = self.rotation.to_rotation_matrix();
        let translation = Mat4::translation(0.0, 0.0, -self.distance);
        translation.multiply(&rot_matrix)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective(self.fov, self.aspect, self.near, self.far)
    }

    pub fn rotate(&mut self, delta: Quaternion) {
        self.rotation = delta.multiply(&self.rotation).normalize();
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    /// Get camera position in world space for ray casting
    pub fn get_position(&self) -> Vec3 {
        // Camera is at (0, 0, distance) rotated by inverse of rotation
        let inv_rot = Quaternion::new(
            -self.rotation.x,
            -self.rotation.y,
            -self.rotation.z,
            self.rotation.w,
        );
        let rot_mat = inv_rot.to_rotation_matrix();

        // Transform (0, 0, distance) by rotation matrix
        Vec3::new(
            rot_mat.data[8] * self.distance,
            rot_mat.data[9] * self.distance,
            rot_mat.data[10] * self.distance,
        )
    }
}
