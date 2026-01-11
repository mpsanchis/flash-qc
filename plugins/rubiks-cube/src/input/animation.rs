use crate::cube::Move;
use crate::math::Mat4;

/// Axis of rotation for a layer
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// Information about which layer is animating
#[derive(Clone, Copy, Debug)]
pub struct AnimatingLayer {
    pub axis: Axis,
    pub layer: i32, // -1, 0, or 1
}

/// State for dragging a face interactively
pub struct FaceDrag {
    pub face: usize,
    pub start_x: f32,
    pub start_y: f32,
    pub current_angle: f32,
    pub layer: Option<AnimatingLayer>,
    pub direction: f32,
    drag_axis_determined: bool,
}

impl FaceDrag {
    pub fn new(face: usize, x: f32, y: f32) -> Self {
        Self {
            face,
            start_x: x,
            start_y: y,
            current_angle: 0.0,
            layer: None,
            direction: 1.0,
            drag_axis_determined: false,
        }
    }

    /// Update drag with new mouse position
    /// Returns true if the drag axis has been determined
    pub fn update(&mut self, x: f32, y: f32) -> bool {
        let dx = x - self.start_x;
        let dy = y - self.start_y;

        // Determine drag direction after threshold
        const THRESHOLD: f32 = 10.0;
        if !self.drag_axis_determined && (dx.abs() > THRESHOLD || dy.abs() > THRESHOLD) {
            self.determine_layer(dx, dy);
            self.drag_axis_determined = true;
            // Reset start position to current for smoother feel
            self.start_x = x;
            self.start_y = y;
        }

        if self.drag_axis_determined {
            // Calculate rotation angle from drag distance
            let drag_dist = match self.layer.as_ref().map(|l| l.axis) {
                Some(Axis::Y) => dx, // Horizontal drag for Y-axis rotation
                _ => -dy,            // Vertical drag for X/Z-axis rotation
            };
            self.current_angle = drag_dist * 0.01 * self.direction;
        }

        self.drag_axis_determined
    }

    fn determine_layer(&mut self, dx: f32, dy: f32) {
        let horizontal = dx.abs() > dy.abs();

        // Determine which layer rotates based on face and drag direction
        // Face indices: 0=U, 1=D, 2=F, 3=B, 4=L, 5=R
        let (axis, layer, dir) = match (self.face, horizontal) {
            // Front face
            (2, true) => (Axis::Y, 0, if dx > 0.0 { -1.0 } else { 1.0 }), // horizontal drag on F -> rotate middle Y
            (2, false) => (Axis::X, 0, if dy > 0.0 { 1.0 } else { -1.0 }), // vertical drag on F -> rotate middle X

            // Back face
            (3, true) => (Axis::Y, 0, if dx > 0.0 { 1.0 } else { -1.0 }),
            (3, false) => (Axis::X, 0, if dy > 0.0 { -1.0 } else { 1.0 }),

            // Up face
            (0, true) => (Axis::Z, 0, if dx > 0.0 { -1.0 } else { 1.0 }),
            (0, false) => (Axis::X, 0, if dy > 0.0 { 1.0 } else { -1.0 }),

            // Down face
            (1, true) => (Axis::Z, 0, if dx > 0.0 { 1.0 } else { -1.0 }),
            (1, false) => (Axis::X, 0, if dy > 0.0 { -1.0 } else { 1.0 }),

            // Left face
            (4, true) => (Axis::Y, 0, if dx > 0.0 { -1.0 } else { 1.0 }),
            (4, false) => (Axis::Z, 0, if dy > 0.0 { 1.0 } else { -1.0 }),

            // Right face
            (5, true) => (Axis::Y, 0, if dx > 0.0 { -1.0 } else { 1.0 }),
            (5, false) => (Axis::Z, 0, if dy > 0.0 { -1.0 } else { 1.0 }),

            _ => (Axis::Y, 0, 1.0),
        };

        // For now, always rotate the face layer itself
        let face_layer = match self.face {
            0 => (Axis::Y, 1),  // U
            1 => (Axis::Y, -1), // D
            2 => (Axis::Z, 1),  // F
            3 => (Axis::Z, -1), // B
            4 => (Axis::X, -1), // L
            5 => (Axis::X, 1),  // R
            _ => (axis, layer),
        };

        self.layer = Some(AnimatingLayer {
            axis: face_layer.0,
            layer: face_layer.1,
        });
        self.direction = dir;
    }

    /// Get the rotation matrix for rendering
    pub fn get_rotation_matrix(&self) -> Mat4 {
        let Some(ref layer) = self.layer else {
            return Mat4::identity();
        };
        let angle = self.current_angle;
        match layer.axis {
            Axis::X => rotation_x(angle),
            Axis::Y => rotation_y(angle),
            Axis::Z => rotation_z(angle),
        }
    }

    /// Check if a cubelet is in the dragging layer
    pub fn is_in_layer(&self, x: i32, y: i32, z: i32) -> bool {
        let Some(ref layer) = self.layer else {
            return false;
        };
        match layer.axis {
            Axis::X => x == layer.layer,
            Axis::Y => y == layer.layer,
            Axis::Z => z == layer.layer,
        }
    }

    /// Finish drag and return the move to apply (if any)
    pub fn finish(&self) -> Option<Move> {
        let layer = self.layer.as_ref()?;

        // Snap to nearest 90 degrees
        let half_pi = std::f32::consts::FRAC_PI_2;
        let snapped_turns = (self.current_angle / half_pi).round() as i32;

        if snapped_turns == 0 {
            return None;
        }

        // Determine move based on layer and rotation count
        // Positive angle = counterclockwise rotation (looking from positive axis)
        // This matches the LayerAnimation convention where:
        //   R has direction -1.0 (negative angle)
        //   R' has direction 1.0 (positive angle)
        let counterclockwise = snapped_turns > 0;
        let double = snapped_turns.abs() >= 2;

        match (layer.axis, layer.layer, counterclockwise, double) {
            // X axis: R (layer=1), L (layer=-1)
            (Axis::X, 1, false, false) => Some(Move::R), // clockwise R
            (Axis::X, 1, true, false) => Some(Move::RPrime), // counterclockwise R'
            (Axis::X, 1, _, true) => Some(Move::R2),
            (Axis::X, -1, false, false) => Some(Move::LPrime), // clockwise L' (from +X view)
            (Axis::X, -1, true, false) => Some(Move::L),       // counterclockwise L
            (Axis::X, -1, _, true) => Some(Move::L2),

            // Y axis: U (layer=1), D (layer=-1)
            (Axis::Y, 1, false, false) => Some(Move::U), // clockwise U
            (Axis::Y, 1, true, false) => Some(Move::UPrime), // counterclockwise U'
            (Axis::Y, 1, _, true) => Some(Move::U2),
            (Axis::Y, -1, false, false) => Some(Move::DPrime), // clockwise D'
            (Axis::Y, -1, true, false) => Some(Move::D),       // counterclockwise D
            (Axis::Y, -1, _, true) => Some(Move::D2),

            // Z axis: F (layer=1), B (layer=-1)
            (Axis::Z, 1, false, false) => Some(Move::F), // clockwise F
            (Axis::Z, 1, true, false) => Some(Move::FPrime), // counterclockwise F'
            (Axis::Z, 1, _, true) => Some(Move::F2),
            (Axis::Z, -1, false, false) => Some(Move::BPrime), // clockwise B'
            (Axis::Z, -1, true, false) => Some(Move::B),       // counterclockwise B
            (Axis::Z, -1, _, true) => Some(Move::B2),

            _ => None,
        }
    }
}

pub struct LayerAnimation {
    pub cube_move: Move,
    pub target_angle: f32,
    pub current_angle: f32,
    pub start_time: f64,
    pub duration: f64,
    pub layer: AnimatingLayer,
    pub direction: f32, // 1.0 or -1.0
}

impl LayerAnimation {
    pub fn new(cube_move: Move, start_time: f64) -> Self {
        let target_angle = match cube_move {
            Move::R2 | Move::L2 | Move::U2 | Move::D2 | Move::F2 | Move::B2 => std::f32::consts::PI,
            _ => std::f32::consts::FRAC_PI_2,
        };

        let (layer, direction) = Self::get_layer_info(cube_move);

        Self {
            cube_move,
            target_angle,
            current_angle: 0.0,
            start_time,
            duration: 200.0, // 200ms
            layer,
            direction,
        }
    }

    fn get_layer_info(cube_move: Move) -> (AnimatingLayer, f32) {
        match cube_move {
            Move::R | Move::R2 => (
                AnimatingLayer {
                    axis: Axis::X,
                    layer: 1,
                },
                -1.0,
            ),
            Move::RPrime => (
                AnimatingLayer {
                    axis: Axis::X,
                    layer: 1,
                },
                1.0,
            ),
            Move::L | Move::L2 => (
                AnimatingLayer {
                    axis: Axis::X,
                    layer: -1,
                },
                1.0,
            ),
            Move::LPrime => (
                AnimatingLayer {
                    axis: Axis::X,
                    layer: -1,
                },
                -1.0,
            ),
            Move::U | Move::U2 => (
                AnimatingLayer {
                    axis: Axis::Y,
                    layer: 1,
                },
                -1.0,
            ),
            Move::UPrime => (
                AnimatingLayer {
                    axis: Axis::Y,
                    layer: 1,
                },
                1.0,
            ),
            Move::D | Move::D2 => (
                AnimatingLayer {
                    axis: Axis::Y,
                    layer: -1,
                },
                1.0,
            ),
            Move::DPrime => (
                AnimatingLayer {
                    axis: Axis::Y,
                    layer: -1,
                },
                -1.0,
            ),
            Move::F | Move::F2 => (
                AnimatingLayer {
                    axis: Axis::Z,
                    layer: 1,
                },
                -1.0,
            ),
            Move::FPrime => (
                AnimatingLayer {
                    axis: Axis::Z,
                    layer: 1,
                },
                1.0,
            ),
            Move::B | Move::B2 => (
                AnimatingLayer {
                    axis: Axis::Z,
                    layer: -1,
                },
                1.0,
            ),
            Move::BPrime => (
                AnimatingLayer {
                    axis: Axis::Z,
                    layer: -1,
                },
                -1.0,
            ),
        }
    }

    /// Update animation and return true if complete
    pub fn update(&mut self, current_time: f64) -> bool {
        let elapsed = current_time - self.start_time;
        let t = (elapsed / self.duration).min(1.0) as f32;
        let eased_t = ease_out_cubic(t);
        self.current_angle = self.target_angle * eased_t;
        t >= 1.0
    }

    /// Get the rotation matrix for the animating layer
    pub fn get_rotation_matrix(&self) -> Mat4 {
        let angle = self.current_angle * self.direction;
        match self.layer.axis {
            Axis::X => rotation_x(angle),
            Axis::Y => rotation_y(angle),
            Axis::Z => rotation_z(angle),
        }
    }

    /// Check if a cubelet at position (x, y, z) is in the animating layer
    pub fn is_in_layer(&self, x: i32, y: i32, z: i32) -> bool {
        match self.layer.axis {
            Axis::X => x == self.layer.layer,
            Axis::Y => y == self.layer.layer,
            Axis::Z => z == self.layer.layer,
        }
    }
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

fn rotation_x(angle: f32) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    #[rustfmt::skip]
    let data = [
        1.0, 0.0, 0.0, 0.0,
        0.0, c, s, 0.0,
        0.0, -s, c, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    Mat4 { data }
}

fn rotation_y(angle: f32) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    #[rustfmt::skip]
    let data = [
        c, 0.0, -s, 0.0,
        0.0, 1.0, 0.0, 0.0,
        s, 0.0, c, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    Mat4 { data }
}

fn rotation_z(angle: f32) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    #[rustfmt::skip]
    let data = [
        c, s, 0.0, 0.0,
        -s, c, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    Mat4 { data }
}
