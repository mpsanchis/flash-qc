use crate::cube::{B, CubeState, D, F, L, R, U};
use crate::input::{FaceDrag, LayerAnimation};
use web_sys::{WebGlBuffer, WebGlRenderingContext};

const CUBELET_SIZE: f32 = 0.95;
const GAP: f32 = 0.025;
const BLACK: [f32; 3] = [0.05, 0.05, 0.05];

pub struct CubeMesh {
    pub vertex_buffer: WebGlBuffer,
    pub index_buffer: WebGlBuffer,
    pub index_count: i32,
}

/// Separate meshes for static and animating parts of the cube
pub struct SplitMesh {
    pub static_mesh: CubeMesh,
    pub layer_mesh: CubeMesh,
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

        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer),
        );
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

impl SplitMesh {
    /// Create split meshes for animated rendering
    /// The layer_mesh contains cubelets in the animating layer
    /// The static_mesh contains all other cubelets
    pub fn new(
        gl: &WebGlRenderingContext,
        state: &CubeState,
        animation: &LayerAnimation,
    ) -> Option<Self> {
        let (static_verts, static_indices, layer_verts, layer_indices) =
            generate_split_geometry(state, animation);

        let static_mesh = create_mesh_buffers(gl, &static_verts, &static_indices)?;
        let layer_mesh = create_mesh_buffers(gl, &layer_verts, &layer_indices)?;

        Some(Self {
            static_mesh,
            layer_mesh,
        })
    }

    /// Create split meshes for face drag rendering
    pub fn from_drag(
        gl: &WebGlRenderingContext,
        state: &CubeState,
        drag: &FaceDrag,
    ) -> Option<Self> {
        let (static_verts, static_indices, layer_verts, layer_indices) =
            generate_drag_split_geometry(state, drag);

        let static_mesh = create_mesh_buffers(gl, &static_verts, &static_indices)?;
        let layer_mesh = create_mesh_buffers(gl, &layer_verts, &layer_indices)?;

        Some(Self {
            static_mesh,
            layer_mesh,
        })
    }
}

fn create_mesh_buffers(
    gl: &WebGlRenderingContext,
    vertices: &[f32],
    indices: &[u16],
) -> Option<CubeMesh> {
    let vertex_buffer = gl.create_buffer()?;
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    unsafe {
        let vert_array = js_sys::Float32Array::view(vertices);
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::DYNAMIC_DRAW,
        );
    }

    let index_buffer = gl.create_buffer()?;
    gl.bind_buffer(
        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&index_buffer),
    );
    unsafe {
        let idx_array = js_sys::Uint16Array::view(indices);
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &idx_array,
            WebGlRenderingContext::DYNAMIC_DRAW,
        );
    }

    Some(CubeMesh {
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as i32,
    })
}

fn generate_split_geometry(
    state: &CubeState,
    animation: &LayerAnimation,
) -> (Vec<f32>, Vec<u16>, Vec<f32>, Vec<u16>) {
    let mut static_vertices: Vec<f32> = Vec::new();
    let mut static_indices: Vec<u16> = Vec::new();
    let mut layer_vertices: Vec<f32> = Vec::new();
    let mut layer_indices: Vec<u16> = Vec::new();

    for x in -1..=1i32 {
        for y in -1..=1i32 {
            for z in -1..=1i32 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }

                if animation.is_in_layer(x, y, z) {
                    let base_index = (layer_vertices.len() / 9) as u16;
                    generate_cubelet(
                        &mut layer_vertices,
                        &mut layer_indices,
                        base_index,
                        x,
                        y,
                        z,
                        state,
                    );
                } else {
                    let base_index = (static_vertices.len() / 9) as u16;
                    generate_cubelet(
                        &mut static_vertices,
                        &mut static_indices,
                        base_index,
                        x,
                        y,
                        z,
                        state,
                    );
                }
            }
        }
    }

    (
        static_vertices,
        static_indices,
        layer_vertices,
        layer_indices,
    )
}

fn generate_drag_split_geometry(
    state: &CubeState,
    drag: &FaceDrag,
) -> (Vec<f32>, Vec<u16>, Vec<f32>, Vec<u16>) {
    let mut static_vertices: Vec<f32> = Vec::new();
    let mut static_indices: Vec<u16> = Vec::new();
    let mut layer_vertices: Vec<f32> = Vec::new();
    let mut layer_indices: Vec<u16> = Vec::new();

    for x in -1..=1i32 {
        for y in -1..=1i32 {
            for z in -1..=1i32 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }

                if drag.is_in_layer(x, y, z) {
                    let base_index = (layer_vertices.len() / 9) as u16;
                    generate_cubelet(
                        &mut layer_vertices,
                        &mut layer_indices,
                        base_index,
                        x,
                        y,
                        z,
                        state,
                    );
                } else {
                    let base_index = (static_vertices.len() / 9) as u16;
                    generate_cubelet(
                        &mut static_vertices,
                        &mut static_indices,
                        base_index,
                        x,
                        y,
                        z,
                        state,
                    );
                }
            }
        }
    }

    (
        static_vertices,
        static_indices,
        layer_vertices,
        layer_indices,
    )
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
                generate_cubelet(&mut vertices, &mut indices, base_index, x, y, z, state);
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
            FaceData {
                p0: [cx - half, cy + half, cz + half],
                p1: [cx + half, cy + half, cz + half],
                p2: [cx + half, cy + half, cz - half],
                p3: [cx - half, cy + half, cz - half],
                normal: [0.0, 1.0, 0.0],
                color,
            },
        );
    }

    // -Y face (bottom) - visible if y == -1
    if y == -1 {
        let color = get_sticker_color(state, D, x, z);
        add_face(
            vertices,
            indices,
            &mut current_index,
            FaceData {
                p0: [cx - half, cy - half, cz - half],
                p1: [cx + half, cy - half, cz - half],
                p2: [cx + half, cy - half, cz + half],
                p3: [cx - half, cy - half, cz + half],
                normal: [0.0, -1.0, 0.0],
                color,
            },
        );
    }

    // +Z face (front) - visible if z == 1
    if z == 1 {
        let color = get_sticker_color(state, F, x, y);
        add_face(
            vertices,
            indices,
            &mut current_index,
            FaceData {
                p0: [cx - half, cy - half, cz + half],
                p1: [cx + half, cy - half, cz + half],
                p2: [cx + half, cy + half, cz + half],
                p3: [cx - half, cy + half, cz + half],
                normal: [0.0, 0.0, 1.0],
                color,
            },
        );
    }

    // -Z face (back) - visible if z == -1
    if z == -1 {
        let color = get_sticker_color(state, B, x, y);
        add_face(
            vertices,
            indices,
            &mut current_index,
            FaceData {
                p0: [cx + half, cy - half, cz - half],
                p1: [cx - half, cy - half, cz - half],
                p2: [cx - half, cy + half, cz - half],
                p3: [cx + half, cy + half, cz - half],
                normal: [0.0, 0.0, -1.0],
                color,
            },
        );
    }

    // -X face (left) - visible if x == -1
    if x == -1 {
        let color = get_sticker_color(state, L, y, z);
        add_face(
            vertices,
            indices,
            &mut current_index,
            FaceData {
                p0: [cx - half, cy - half, cz - half],
                p1: [cx - half, cy - half, cz + half],
                p2: [cx - half, cy + half, cz + half],
                p3: [cx - half, cy + half, cz - half],
                normal: [-1.0, 0.0, 0.0],
                color,
            },
        );
    }

    // +X face (right) - visible if x == 1
    if x == 1 {
        let color = get_sticker_color(state, R, y, z);
        add_face(
            vertices,
            indices,
            &mut current_index,
            FaceData {
                p0: [cx + half, cy - half, cz + half],
                p1: [cx + half, cy - half, cz - half],
                p2: [cx + half, cy + half, cz - half],
                p3: [cx + half, cy + half, cz + half],
                normal: [1.0, 0.0, 0.0],
                color,
            },
        );
    }

    // Add black edges for internal faces
    // Internal +Y
    if y != 1 {
        add_face(
            vertices,
            indices,
            &mut current_index,
            FaceData {
                p0: [cx - half, cy + half + GAP, cz + half],
                p1: [cx + half, cy + half + GAP, cz + half],
                p2: [cx + half, cy + half + GAP, cz - half],
                p3: [cx - half, cy + half + GAP, cz - half],
                normal: [0.0, 1.0, 0.0],
                color: BLACK,
            },
        );
    }
    // Internal -Y
    if y != -1 {
        add_face(
            vertices,
            indices,
            &mut current_index,
            FaceData {
                p0: [cx - half, cy - half - GAP, cz - half],
                p1: [cx + half, cy - half - GAP, cz - half],
                p2: [cx + half, cy - half - GAP, cz + half],
                p3: [cx - half, cy - half - GAP, cz + half],
                normal: [0.0, -1.0, 0.0],
                color: BLACK,
            },
        );
    }
    // Internal +Z
    if z != 1 {
        add_face(
            vertices,
            indices,
            &mut current_index,
            FaceData {
                p0: [cx - half, cy - half, cz + half + GAP],
                p1: [cx + half, cy - half, cz + half + GAP],
                p2: [cx + half, cy + half, cz + half + GAP],
                p3: [cx - half, cy + half, cz + half + GAP],
                normal: [0.0, 0.0, 1.0],
                color: BLACK,
            },
        );
    }
    // Internal -Z
    if z != -1 {
        add_face(
            vertices,
            indices,
            &mut current_index,
            FaceData {
                p0: [cx + half, cy - half, cz - half - GAP],
                p1: [cx - half, cy - half, cz - half - GAP],
                p2: [cx - half, cy + half, cz - half - GAP],
                p3: [cx + half, cy + half, cz - half - GAP],
                normal: [0.0, 0.0, -1.0],
                color: BLACK,
            },
        );
    }
    // Internal -X
    if x != -1 {
        add_face(
            vertices,
            indices,
            &mut current_index,
            FaceData {
                p0: [cx - half - GAP, cy - half, cz - half],
                p1: [cx - half - GAP, cy - half, cz + half],
                p2: [cx - half - GAP, cy + half, cz + half],
                p3: [cx - half - GAP, cy + half, cz - half],
                normal: [-1.0, 0.0, 0.0],
                color: BLACK,
            },
        );
    }
    // Internal +X
    if x != 1 {
        add_face(
            vertices,
            indices,
            &mut current_index,
            FaceData {
                p0: [cx + half + GAP, cy - half, cz + half],
                p1: [cx + half + GAP, cy - half, cz - half],
                p2: [cx + half + GAP, cy + half, cz - half],
                p3: [cx + half + GAP, cy + half, cz + half],
                normal: [1.0, 0.0, 0.0],
                color: BLACK,
            },
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

struct FaceData {
    p0: [f32; 3],
    p1: [f32; 3],
    p2: [f32; 3],
    p3: [f32; 3],
    normal: [f32; 3],
    color: [f32; 3],
}

fn add_face(
    vertices: &mut Vec<f32>,
    indices: &mut Vec<u16>,
    current_index: &mut u16,
    face: FaceData,
) {
    // Add 4 vertices
    for p in [face.p0, face.p1, face.p2, face.p3] {
        vertices.extend_from_slice(&p);
        vertices.extend_from_slice(&face.normal);
        vertices.extend_from_slice(&face.color);
    }

    // Add 2 triangles (6 indices)
    let i = *current_index;
    indices.extend_from_slice(&[i, i + 1, i + 2, i, i + 2, i + 3]);
    *current_index += 4;
}
