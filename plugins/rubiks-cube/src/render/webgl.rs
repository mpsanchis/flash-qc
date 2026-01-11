use crate::cube::CubeState;
use crate::math::Mat4;
use crate::render::{Camera, CubeMesh};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};

use super::shaders::{FRAGMENT_SHADER, VERTEX_SHADER};

pub struct Renderer {
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    mesh: CubeMesh,
    u_model: WebGlUniformLocation,
    u_view: WebGlUniformLocation,
    u_projection: WebGlUniformLocation,
    u_light_dir: WebGlUniformLocation,
}

impl Renderer {
    pub fn new(canvas: &HtmlCanvasElement, state: &CubeState) -> Result<Self, String> {
        let gl = canvas
            .get_context("webgl")
            .map_err(|_| "Failed to get WebGL context")?
            .ok_or("WebGL not supported")?
            .dyn_into::<WebGlRenderingContext>()
            .map_err(|_| "Failed to cast to WebGlRenderingContext")?;

        gl.enable(WebGlRenderingContext::DEPTH_TEST);
        gl.enable(WebGlRenderingContext::CULL_FACE);
        gl.cull_face(WebGlRenderingContext::BACK);
        gl.clear_color(0.12, 0.12, 0.14, 1.0);

        let program = create_program(&gl)?;
        gl.use_program(Some(&program));

        let mesh = CubeMesh::new(&gl, state).ok_or("Failed to create mesh")?;

        // Get uniform locations
        let u_model = gl
            .get_uniform_location(&program, "u_model")
            .ok_or("u_model not found")?;
        let u_view = gl
            .get_uniform_location(&program, "u_view")
            .ok_or("u_view not found")?;
        let u_projection = gl
            .get_uniform_location(&program, "u_projection")
            .ok_or("u_projection not found")?;
        let u_light_dir = gl
            .get_uniform_location(&program, "u_light_dir")
            .ok_or("u_light_dir not found")?;

        // Set up vertex attributes
        setup_attributes(&gl, &program);

        Ok(Self {
            gl,
            program,
            mesh,
            u_model,
            u_view,
            u_projection,
            u_light_dir,
        })
    }

    pub fn render(&self, camera: &Camera, model: &Mat4) {
        let gl = &self.gl;

        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        // Set uniforms
        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_model), false, model.as_slice());
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_view),
            false,
            camera.view_matrix().as_slice(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_projection),
            false,
            camera.projection_matrix().as_slice(),
        );
        gl.uniform3f(Some(&self.u_light_dir), 0.5, 0.7, 1.0);

        // Bind buffers and draw
        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.mesh.vertex_buffer),
        );
        gl.bind_buffer(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&self.mesh.index_buffer),
        );

        setup_attributes(&self.gl, &self.program);

        gl.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            self.mesh.index_count,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    pub fn update_mesh(&mut self, state: &CubeState) {
        self.mesh.update(&self.gl, state);
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.gl.viewport(0, 0, width as i32, height as i32);
    }
}

fn create_program(gl: &WebGlRenderingContext) -> Result<WebGlProgram, String> {
    let vert_shader = compile_shader(gl, WebGlRenderingContext::VERTEX_SHADER, VERTEX_SHADER)?;
    let frag_shader = compile_shader(gl, WebGlRenderingContext::FRAGMENT_SHADER, FRAGMENT_SHADER)?;

    let program = gl.create_program().ok_or("Failed to create program")?;
    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if !gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let info = gl.get_program_info_log(&program).unwrap_or_default();
        return Err(format!("Program link error: {}", info));
    }

    Ok(program)
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<web_sys::WebGlShader, String> {
    let shader = gl.create_shader(shader_type).ok_or("Failed to create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if !gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let info = gl.get_shader_info_log(&shader).unwrap_or_default();
        return Err(format!("Shader compile error: {}", info));
    }

    Ok(shader)
}

fn setup_attributes(gl: &WebGlRenderingContext, program: &WebGlProgram) {
    let stride = 9 * 4; // 9 floats per vertex (pos, normal, color)

    let a_position = gl.get_attrib_location(program, "a_position") as u32;
    gl.enable_vertex_attrib_array(a_position);
    gl.vertex_attrib_pointer_with_i32(a_position, 3, WebGlRenderingContext::FLOAT, false, stride, 0);

    let a_normal = gl.get_attrib_location(program, "a_normal") as u32;
    gl.enable_vertex_attrib_array(a_normal);
    gl.vertex_attrib_pointer_with_i32(
        a_normal,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        stride,
        3 * 4,
    );

    let a_color = gl.get_attrib_location(program, "a_color") as u32;
    gl.enable_vertex_attrib_array(a_color);
    gl.vertex_attrib_pointer_with_i32(
        a_color,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        stride,
        6 * 4,
    );
}
