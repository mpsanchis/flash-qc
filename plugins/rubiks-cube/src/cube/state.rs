use super::Move;
use serde::{Deserialize, Serialize};

/// Face colors matching standard Rubik's cube convention
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Color {
    White = 0,  // Up (U)
    Yellow = 1, // Down (D)
    Green = 2,  // Front (F)
    Blue = 3,   // Back (B)
    Orange = 4, // Left (L)
    Red = 5,    // Right (R)
}

impl Color {
    pub fn to_rgb(self) -> [f32; 3] {
        match self {
            Color::White => [1.0, 1.0, 1.0],
            Color::Yellow => [1.0, 0.85, 0.0],
            Color::Green => [0.0, 0.62, 0.38],
            Color::Blue => [0.0, 0.32, 0.73],
            Color::Orange => [1.0, 0.35, 0.0],
            Color::Red => [0.72, 0.07, 0.2],
        }
    }
}

/// Each face has 9 stickers in reading order:
/// 0 1 2
/// 3 4 5
/// 6 7 8
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Face {
    pub stickers: [Color; 9],
}

impl Face {
    pub fn new(color: Color) -> Self {
        Self {
            stickers: [color; 9],
        }
    }

    /// Rotate face clockwise
    pub fn rotate_cw(&mut self) {
        let old = self.stickers;
        self.stickers = [
            old[6], old[3], old[0], old[7], old[4], old[1], old[8], old[5], old[2],
        ];
    }

    /// Check if all stickers are the same color
    pub fn is_solved(&self) -> bool {
        let center = self.stickers[4];
        self.stickers.iter().all(|&s| s == center)
    }
}

/// Face indices: 0=U, 1=D, 2=F, 3=B, 4=L, 5=R
pub const U: usize = 0;
pub const D: usize = 1;
pub const F: usize = 2;
pub const B: usize = 3;
pub const L: usize = 4;
pub const R: usize = 5;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CubeState {
    pub faces: [Face; 6],
}

impl CubeState {
    pub fn solved() -> Self {
        Self {
            faces: [
                Face::new(Color::White),  // U
                Face::new(Color::Yellow), // D
                Face::new(Color::Green),  // F
                Face::new(Color::Blue),   // B
                Face::new(Color::Orange), // L
                Face::new(Color::Red),    // R
            ],
        }
    }

    pub fn is_solved(&self) -> bool {
        self.faces.iter().all(|face| face.is_solved())
    }

    pub fn apply_move(&mut self, m: Move) {
        match m {
            Move::R => self.move_r(),
            Move::RPrime => self.move_r_prime(),
            Move::R2 => {
                self.move_r();
                self.move_r();
            }
            Move::L => self.move_l(),
            Move::LPrime => self.move_l_prime(),
            Move::L2 => {
                self.move_l();
                self.move_l();
            }
            Move::U => self.move_u(),
            Move::UPrime => self.move_u_prime(),
            Move::U2 => {
                self.move_u();
                self.move_u();
            }
            Move::D => self.move_d(),
            Move::DPrime => self.move_d_prime(),
            Move::D2 => {
                self.move_d();
                self.move_d();
            }
            Move::F => self.move_f(),
            Move::FPrime => self.move_f_prime(),
            Move::F2 => {
                self.move_f();
                self.move_f();
            }
            Move::B => self.move_b(),
            Move::BPrime => self.move_b_prime(),
            Move::B2 => {
                self.move_b();
                self.move_b();
            }
        }
    }

    fn move_r(&mut self) {
        self.faces[R].rotate_cw();
        let temp = [
            self.faces[F].stickers[2],
            self.faces[F].stickers[5],
            self.faces[F].stickers[8],
        ];
        // F right column -> U right column
        self.faces[F].stickers[2] = self.faces[D].stickers[2];
        self.faces[F].stickers[5] = self.faces[D].stickers[5];
        self.faces[F].stickers[8] = self.faces[D].stickers[8];
        // D right column -> B left column (reversed)
        self.faces[D].stickers[2] = self.faces[B].stickers[6];
        self.faces[D].stickers[5] = self.faces[B].stickers[3];
        self.faces[D].stickers[8] = self.faces[B].stickers[0];
        // B left column -> U right column (reversed)
        self.faces[B].stickers[6] = self.faces[U].stickers[2];
        self.faces[B].stickers[3] = self.faces[U].stickers[5];
        self.faces[B].stickers[0] = self.faces[U].stickers[8];
        // U right column -> F right column (from temp)
        self.faces[U].stickers[2] = temp[0];
        self.faces[U].stickers[5] = temp[1];
        self.faces[U].stickers[8] = temp[2];
    }

    fn move_r_prime(&mut self) {
        self.move_r();
        self.move_r();
        self.move_r();
    }

    fn move_l(&mut self) {
        self.faces[L].rotate_cw();
        let temp = [
            self.faces[F].stickers[0],
            self.faces[F].stickers[3],
            self.faces[F].stickers[6],
        ];
        // U left column -> F left column
        self.faces[F].stickers[0] = self.faces[U].stickers[0];
        self.faces[F].stickers[3] = self.faces[U].stickers[3];
        self.faces[F].stickers[6] = self.faces[U].stickers[6];
        // B right column -> U left column (reversed)
        self.faces[U].stickers[0] = self.faces[B].stickers[8];
        self.faces[U].stickers[3] = self.faces[B].stickers[5];
        self.faces[U].stickers[6] = self.faces[B].stickers[2];
        // D left column -> B right column (reversed)
        self.faces[B].stickers[8] = self.faces[D].stickers[0];
        self.faces[B].stickers[5] = self.faces[D].stickers[3];
        self.faces[B].stickers[2] = self.faces[D].stickers[6];
        // F left column -> D left column (from temp)
        self.faces[D].stickers[0] = temp[0];
        self.faces[D].stickers[3] = temp[1];
        self.faces[D].stickers[6] = temp[2];
    }

    fn move_l_prime(&mut self) {
        self.move_l();
        self.move_l();
        self.move_l();
    }

    fn move_u(&mut self) {
        self.faces[U].rotate_cw();
        let temp = [
            self.faces[F].stickers[0],
            self.faces[F].stickers[1],
            self.faces[F].stickers[2],
        ];
        // R top row -> F top row
        self.faces[F].stickers[0] = self.faces[R].stickers[0];
        self.faces[F].stickers[1] = self.faces[R].stickers[1];
        self.faces[F].stickers[2] = self.faces[R].stickers[2];
        // B top row -> R top row
        self.faces[R].stickers[0] = self.faces[B].stickers[0];
        self.faces[R].stickers[1] = self.faces[B].stickers[1];
        self.faces[R].stickers[2] = self.faces[B].stickers[2];
        // L top row -> B top row
        self.faces[B].stickers[0] = self.faces[L].stickers[0];
        self.faces[B].stickers[1] = self.faces[L].stickers[1];
        self.faces[B].stickers[2] = self.faces[L].stickers[2];
        // F top row -> L top row (from temp)
        self.faces[L].stickers[0] = temp[0];
        self.faces[L].stickers[1] = temp[1];
        self.faces[L].stickers[2] = temp[2];
    }

    fn move_u_prime(&mut self) {
        self.move_u();
        self.move_u();
        self.move_u();
    }

    fn move_d(&mut self) {
        self.faces[D].rotate_cw();
        let temp = [
            self.faces[F].stickers[6],
            self.faces[F].stickers[7],
            self.faces[F].stickers[8],
        ];
        // L bottom row -> F bottom row
        self.faces[F].stickers[6] = self.faces[L].stickers[6];
        self.faces[F].stickers[7] = self.faces[L].stickers[7];
        self.faces[F].stickers[8] = self.faces[L].stickers[8];
        // B bottom row -> L bottom row
        self.faces[L].stickers[6] = self.faces[B].stickers[6];
        self.faces[L].stickers[7] = self.faces[B].stickers[7];
        self.faces[L].stickers[8] = self.faces[B].stickers[8];
        // R bottom row -> B bottom row
        self.faces[B].stickers[6] = self.faces[R].stickers[6];
        self.faces[B].stickers[7] = self.faces[R].stickers[7];
        self.faces[B].stickers[8] = self.faces[R].stickers[8];
        // F bottom row -> R bottom row (from temp)
        self.faces[R].stickers[6] = temp[0];
        self.faces[R].stickers[7] = temp[1];
        self.faces[R].stickers[8] = temp[2];
    }

    fn move_d_prime(&mut self) {
        self.move_d();
        self.move_d();
        self.move_d();
    }

    fn move_f(&mut self) {
        self.faces[F].rotate_cw();
        let temp = [
            self.faces[U].stickers[6],
            self.faces[U].stickers[7],
            self.faces[U].stickers[8],
        ];
        // L right column -> U bottom row
        self.faces[U].stickers[6] = self.faces[L].stickers[8];
        self.faces[U].stickers[7] = self.faces[L].stickers[5];
        self.faces[U].stickers[8] = self.faces[L].stickers[2];
        // D top row -> L right column
        self.faces[L].stickers[2] = self.faces[D].stickers[0];
        self.faces[L].stickers[5] = self.faces[D].stickers[1];
        self.faces[L].stickers[8] = self.faces[D].stickers[2];
        // R left column -> D top row (reversed)
        self.faces[D].stickers[0] = self.faces[R].stickers[6];
        self.faces[D].stickers[1] = self.faces[R].stickers[3];
        self.faces[D].stickers[2] = self.faces[R].stickers[0];
        // U bottom row -> R left column (from temp)
        self.faces[R].stickers[0] = temp[0];
        self.faces[R].stickers[3] = temp[1];
        self.faces[R].stickers[6] = temp[2];
    }

    fn move_f_prime(&mut self) {
        self.move_f();
        self.move_f();
        self.move_f();
    }

    fn move_b(&mut self) {
        self.faces[B].rotate_cw();
        let temp = [
            self.faces[U].stickers[0],
            self.faces[U].stickers[1],
            self.faces[U].stickers[2],
        ];
        // R right column -> U top row
        self.faces[U].stickers[0] = self.faces[R].stickers[2];
        self.faces[U].stickers[1] = self.faces[R].stickers[5];
        self.faces[U].stickers[2] = self.faces[R].stickers[8];
        // D bottom row -> R right column (reversed)
        self.faces[R].stickers[2] = self.faces[D].stickers[8];
        self.faces[R].stickers[5] = self.faces[D].stickers[7];
        self.faces[R].stickers[8] = self.faces[D].stickers[6];
        // L left column -> D bottom row
        self.faces[D].stickers[6] = self.faces[L].stickers[0];
        self.faces[D].stickers[7] = self.faces[L].stickers[3];
        self.faces[D].stickers[8] = self.faces[L].stickers[6];
        // U top row -> L left column (reversed, from temp)
        self.faces[L].stickers[0] = temp[2];
        self.faces[L].stickers[3] = temp[1];
        self.faces[L].stickers[6] = temp[0];
    }

    fn move_b_prime(&mut self) {
        self.move_b();
        self.move_b();
        self.move_b();
    }
}

impl Default for CubeState {
    fn default() -> Self {
        Self::solved()
    }
}
