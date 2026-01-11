use crate::cube::{CubeState, B, D, F, L, R, U};
use web_sys::{WebGlBuffer, WebGlRenderingContext};

const CUBELET_SIZE: f32 = 0.95;
const GAP: f32 = 0.025;
const BLACK: [f32; 3] = [0.05, 0.05, 0.05];

pub struct CubeMesh {
    pub vertex_buffer: WebGlBuffer,
    pub index_buffer: WebGlBuffer,
    pub index_count: i32,
}

impl CubeMesh {
    pub fn new(gl: &WebGlRenderingContext, state: &CubeState) -> Option<Self> {
        let (vertices, indices) = generate_cube_geometry(state);

        let vertex_buffer = gl.create_buffer()?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let index_buffer = gl.create_buffer()?;
        gl.bind_buffer(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&index_buffer),
        );
        unsafe {
            let idx_array = js_sys::Uint16Array::view(&indices);
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &idx_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        Some(Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as i32,
        })
    }

    pub fn update(&mut self, gl: &WebGlRenderingContext, state: &CubeState) {
        let (vertices, _) = generate_cube_geometry(state);

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.vertex_buffer));
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }
    }
}

fn generate_cube_geometry(state: &CubeState) -> (Vec<f32>, Vec<u16>) {
    let mut vertices: Vec<f32> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    // Generate 27 cubelets (3x3x3 grid)
    for x in -1..=1i32 {
        for y in -1..=1i32 {
            for z in -1..=1i32 {
                // Skip center cubelet (not visible)
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }

                let base_index = (vertices.len() / 9) as u16;
                generate_cubelet(
                    &mut vertices,
                    &mut indices,
                    base_index,
                    x,
                    y,
                    z,
                    state,
                );
            }
        }
    }

    (vertices, indices)
}

fn generate_cubelet(
    vertices: &mut Vec<f32>,
    indices: &mut Vec<u16>,
    base_index: u16,
    x: i32,
    y: i32,
    z: i32,
    state: &CubeState,
) {
    let cx = x as f32;
    let cy = y as f32;
    let cz = z as f32;
    let half = CUBELET_SIZE / 2.0;

    // Each cubelet has 6 faces, but we only render visible external faces
    let mut current_index = base_index;

    // +Y face (top) - visible if y == 1
    if y == 1 {
        let color = get_sticker_color(state, U, x, z);
        add_face(
            vertices,
            indices,
            &mut current_index,
            [cx - half, cy + half, cz + half],
            [cx + half, cy + half, cz + half],
            [cx + half, cy + half, cz - half],
            [cx - half, cy + half, cz - half],
            [0.0, 1.0, 0.0],
            color,
        );
    }

    // -Y face (bottom) - visible if y == -1
    if y == -1 {
        let color = get_sticker_color(state, D, x, z);
        add_face(
            vertices,
            indices,
            &mut current_index,
            [cx - half, cy - half, cz - half],
            [cx + half, cy - half, cz - half],
            [cx + half, cy - half, cz + half],
            [cx - half, cy - half, cz + half],
            [0.0, -1.0, 0.0],
            color,
        );
    }

    // +Z face (front) - visible if z == 1
    if z == 1 {
        let color = get_sticker_color(state, F, x, y);
        add_face(
            vertices,
            indices,
            &mut current_index,
            [cx - half, cy - half, cz + half],
            [cx + half, cy - half, cz + half],
            [cx + half, cy + half, cz + half],
            [cx - half, cy + half, cz + half],
            [0.0, 0.0, 1.0],
            color,
        );
    }

    // -Z face (back) - visible if z == -1
    if z == -1 {
        let color = get_sticker_color(state, B, x, y);
        add_face(
            vertices,
            indices,
            &mut current_index,
            [cx + half, cy - half, cz - half],
            [cx - half, cy - half, cz - half],
            [cx - half, cy + half, cz - half],
            [cx + half, cy + half, cz - half],
            [0.0, 0.0, -1.0],
            color,
        );
    }

    // -X face (left) - visible if x == -1
    if x == -1 {
        let color = get_sticker_color(state, L, y, z);
        add_face(
            vertices,
            indices,
            &mut current_index,
            [cx - half, cy - half, cz - half],
            [cx - half, cy - half, cz + half],
            [cx - half, cy + half, cz + half],
            [cx - half, cy + half, cz - half],
            [-1.0, 0.0, 0.0],
            color,
        );
    }

    // +X face (right) - visible if x == 1
    if x == 1 {
        let color = get_sticker_color(state, R, y, z);
        add_face(
            vertices,
            indices,
            &mut current_index,
            [cx + half, cy - half, cz + half],
            [cx + half, cy - half, cz - half],
            [cx + half, cy + half, cz - half],
            [cx + half, cy + half, cz + half],
            [1.0, 0.0, 0.0],
            color,
        );
    }

    // Add black edges for internal faces
    // Internal +Y
    if y != 1 {
        add_face(
            vertices,
            indices,
            &mut current_index,
            [cx - half, cy + half + GAP, cz + half],
            [cx + half, cy + half + GAP, cz + half],
            [cx + half, cy + half + GAP, cz - half],
            [cx - half, cy + half + GAP, cz - half],
            [0.0, 1.0, 0.0],
            BLACK,
        );
    }
    // Internal -Y
    if y != -1 {
        add_face(
            vertices,
            indices,
            &mut current_index,
            [cx - half, cy - half - GAP, cz - half],
            [cx + half, cy - half - GAP, cz - half],
            [cx + half, cy - half - GAP, cz + half],
            [cx - half, cy - half - GAP, cz + half],
            [0.0, -1.0, 0.0],
            BLACK,
        );
    }
    // Internal +Z
    if z != 1 {
        add_face(
            vertices,
            indices,
            &mut current_index,
            [cx - half, cy - half, cz + half + GAP],
            [cx + half, cy - half, cz + half + GAP],
            [cx + half, cy + half, cz + half + GAP],
            [cx - half, cy + half, cz + half + GAP],
            [0.0, 0.0, 1.0],
            BLACK,
        );
    }
    // Internal -Z
    if z != -1 {
        add_face(
            vertices,
            indices,
            &mut current_index,
            [cx + half, cy - half, cz - half - GAP],
            [cx - half, cy - half, cz - half - GAP],
            [cx - half, cy + half, cz - half - GAP],
            [cx + half, cy + half, cz - half - GAP],
            [0.0, 0.0, -1.0],
            BLACK,
        );
    }
    // Internal -X
    if x != -1 {
        add_face(
            vertices,
            indices,
            &mut current_index,
            [cx - half - GAP, cy - half, cz - half],
            [cx - half - GAP, cy - half, cz + half],
            [cx - half - GAP, cy + half, cz + half],
            [cx - half - GAP, cy + half, cz - half],
            [-1.0, 0.0, 0.0],
            BLACK,
        );
    }
    // Internal +X
    if x != 1 {
        add_face(
            vertices,
            indices,
            &mut current_index,
            [cx + half + GAP, cy - half, cz + half],
            [cx + half + GAP, cy - half, cz - half],
            [cx + half + GAP, cy + half, cz - half],
            [cx + half + GAP, cy + half, cz + half],
            [1.0, 0.0, 0.0],
            BLACK,
        );
    }
}

fn get_sticker_color(state: &CubeState, face: usize, a: i32, b: i32) -> [f32; 3] {
    // Convert cubelet coordinates to sticker index
    // a and b are in range [-1, 1], need to map to [0, 2] for sticker index
    let sticker_index = match face {
        U => {
            // U face: a=x, b=z, looking down at it
            // x: -1->0, 0->1, 1->2 (left to right)
            // z: 1->0, 0->1, -1->2 (front to back becomes top to bottom)
            let col = (a + 1) as usize;
            let row = (1 - b) as usize;
            row * 3 + col
        }
        D => {
            // D face: a=x, b=z, looking up at it
            let col = (a + 1) as usize;
            let row = (b + 1) as usize;
            row * 3 + col
        }
        F => {
            // F face: a=x, b=y
            let col = (a + 1) as usize;
            let row = (1 - b) as usize;
            row * 3 + col
        }
        B => {
            // B face: a=x, b=y (mirrored horizontally)
            let col = (1 - a) as usize;
            let row = (1 - b) as usize;
            row * 3 + col
        }
        L => {
            // L face: a=y, b=z
            let col = (b + 1) as usize;
            let row = (1 - a) as usize;
            row * 3 + col
        }
        R => {
            // R face: a=y, b=z (mirrored)
            let col = (1 - b) as usize;
            let row = (1 - a) as usize;
            row * 3 + col
        }
        _ => 0,
    };

    state.faces[face].stickers[sticker_index].to_rgb()
}

fn add_face(
    vertices: &mut Vec<f32>,
    indices: &mut Vec<u16>,
    current_index: &mut u16,
    p0: [f32; 3],
    p1: [f32; 3],
    p2: [f32; 3],
    p3: [f32; 3],
    normal: [f32; 3],
    color: [f32; 3],
) {
    // Add 4 vertices
    for p in [p0, p1, p2, p3] {
        vertices.extend_from_slice(&p);
        vertices.extend_from_slice(&normal);
        vertices.extend_from_slice(&color);
    }

    // Add 2 triangles (6 indices)
    let i = *current_index;
    indices.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
    *current_index += 4;
}
