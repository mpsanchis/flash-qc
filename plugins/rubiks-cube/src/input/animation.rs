use crate::cube::Move;

pub struct LayerAnimation {
    pub cube_move: Move,
    pub target_angle: f32,
    pub current_angle: f32,
    pub start_time: f64,
    pub duration: f64,
}

impl LayerAnimation {
    pub fn new(cube_move: Move, start_time: f64) -> Self {
        let target_angle = match cube_move {
            Move::R2 | Move::L2 | Move::U2 | Move::D2 | Move::F2 | Move::B2 => {
                std::f32::consts::PI
            }
            _ => std::f32::consts::FRAC_PI_2,
        };

        Self {
            cube_move,
            target_angle,
            current_angle: 0.0,
            start_time,
            duration: 200.0, // 200ms
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
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}
