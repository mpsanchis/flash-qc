/// 4x4 matrix stored in column-major order (OpenGL convention)
#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub data: [f32; 16],
}

impl Mat4 {
    pub fn identity() -> Self {
        #[rustfmt::skip]
        let data = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        Self { data }
    }

    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y / 2.0).tan();
        let nf = 1.0 / (near - far);

        #[rustfmt::skip]
        let data = [
            f / aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (far + near) * nf, -1.0,
            0.0, 0.0, 2.0 * far * near * nf, 0.0,
        ];
        Self { data }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        #[rustfmt::skip]
        let data = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            x, y, z, 1.0,
        ];
        Self { data }
    }

    pub fn multiply(&self, other: &Mat4) -> Mat4 {
        let mut result = [0.0f32; 16];

        for col in 0..4 {
            for row in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.data[k * 4 + row] * other.data[col * 4 + k];
                }
                result[col * 4 + row] = sum;
            }
        }

        Mat4 { data: result }
    }

    pub fn as_slice(&self) -> &[f32; 16] {
        &self.data
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::identity()
    }
}
