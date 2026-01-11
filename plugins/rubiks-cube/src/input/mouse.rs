use crate::math::{Quaternion, Vec3};

pub struct MouseHandler {
    is_dragging: bool,
    last_x: f32,
    last_y: f32,
    sensitivity: f32,
}

impl MouseHandler {
    pub fn new() -> Self {
        Self {
            is_dragging: false,
            last_x: 0.0,
            last_y: 0.0,
            sensitivity: 0.01,
        }
    }

    pub fn start_drag(&mut self, x: f32, y: f32) {
        self.is_dragging = true;
        self.last_x = x;
        self.last_y = y;
    }

    pub fn end_drag(&mut self) {
        self.is_dragging = false;
    }

    pub fn drag(&mut self, x: f32, y: f32) -> Option<Quaternion> {
        if !self.is_dragging {
            return None;
        }

        let dx = x - self.last_x;
        let dy = y - self.last_y;

        self.last_x = x;
        self.last_y = y;

        if dx.abs() < 0.001 && dy.abs() < 0.001 {
            return None;
        }

        // Create rotation quaternion from mouse movement
        // Horizontal movement rotates around Y axis
        // Vertical movement rotates around X axis
        let angle_y = -dx * self.sensitivity;
        let angle_x = -dy * self.sensitivity;

        let rot_y = Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), angle_y);
        let rot_x = Quaternion::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), angle_x);

        Some(rot_x.multiply(&rot_y))
    }
}

impl Default for MouseHandler {
    fn default() -> Self {
        Self::new()
    }
}
