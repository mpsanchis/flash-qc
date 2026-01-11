mod cube;
mod input;
mod math;
mod render;

use std::cell::RefCell;
use std::rc::Rc;

use cube::{CubeState, Move, parse_scramble};
use input::{FaceDrag, LayerAnimation, MouseHandler};
use math::Mat4;
use render::{Camera, RayPicker, Renderer};

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, MouseEvent};

#[wasm_bindgen]
pub struct RubiksCube {
    canvas: HtmlCanvasElement,
    renderer: Renderer,
    camera: Rc<RefCell<Camera>>,
    state: Rc<RefCell<CubeState>>,
    mouse: Rc<RefCell<MouseHandler>>,
    animation: Rc<RefCell<Option<LayerAnimation>>>,
    face_drag: Rc<RefCell<Option<FaceDrag>>>,
    move_count: Rc<RefCell<u32>>,
    needs_mesh_update: Rc<RefCell<bool>>,
    closures: Vec<Closure<dyn FnMut(MouseEvent)>>,
}

#[wasm_bindgen]
impl RubiksCube {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<RubiksCube, JsValue> {
        let window = web_sys::window().ok_or("No window")?;
        let document = window.document().ok_or("No document")?;
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas not found")?
            .dyn_into::<HtmlCanvasElement>()?;

        let state = CubeState::solved();
        let aspect = canvas.width() as f32 / canvas.height() as f32;
        let camera = Camera::new(aspect);

        let renderer = Renderer::new(&canvas, &state).map_err(|e| JsValue::from_str(&e))?;

        Ok(Self {
            canvas,
            renderer,
            camera: Rc::new(RefCell::new(camera)),
            state: Rc::new(RefCell::new(state)),
            mouse: Rc::new(RefCell::new(MouseHandler::new())),
            animation: Rc::new(RefCell::new(None)),
            face_drag: Rc::new(RefCell::new(None)),
            move_count: Rc::new(RefCell::new(0)),
            needs_mesh_update: Rc::new(RefCell::new(false)),
            closures: Vec::new(),
        })
    }

    pub fn setup_event_listeners(&mut self) -> Result<(), JsValue> {
        let canvas = &self.canvas;
        let canvas_ref = canvas.clone();

        // Mouse down - start face drag or camera drag
        {
            let mouse = self.mouse.clone();
            let camera = self.camera.clone();
            let animation = self.animation.clone();
            let face_drag = self.face_drag.clone();
            let canvas_clone = canvas_ref.clone();

            let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                let x = event.offset_x() as f32;
                let y = event.offset_y() as f32;
                let canvas_width = canvas_clone.width() as f32;
                let canvas_height = canvas_clone.height() as f32;

                // If animation is running, ignore input
                if animation.borrow().is_some() {
                    return;
                }

                // Try to pick a face first
                let cam = camera.borrow();
                if let Some(hit) = RayPicker::pick(&cam, x, y, canvas_width, canvas_height) {
                    // Start face drag
                    *face_drag.borrow_mut() = Some(FaceDrag::new(hit.face, x, y));
                } else {
                    // Start drag for camera rotation
                    mouse.borrow_mut().start_drag(x, y);
                }
            }) as Box<dyn FnMut(MouseEvent)>);

            canvas
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
            self.closures.push(closure);
        }

        // Mouse move - face drag or camera rotation
        {
            let mouse = self.mouse.clone();
            let camera = self.camera.clone();
            let face_drag = self.face_drag.clone();

            let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                let x = event.offset_x() as f32;
                let y = event.offset_y() as f32;

                // Check if we're doing a face drag
                if let Some(ref mut drag) = *face_drag.borrow_mut() {
                    drag.update(x, y);
                    return;
                }

                // Otherwise, camera rotation
                if let Some(delta) = mouse.borrow_mut().drag(x, y) {
                    camera.borrow_mut().rotate(delta);
                }
            }) as Box<dyn FnMut(MouseEvent)>);

            canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            self.closures.push(closure);
        }

        // Mouse up - finish face drag or end camera drag
        {
            let mouse = self.mouse.clone();
            let face_drag = self.face_drag.clone();
            let state = self.state.clone();
            let move_count = self.move_count.clone();
            let needs_mesh_update = self.needs_mesh_update.clone();

            let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                // Finish face drag if active
                if let Some(drag) = face_drag.borrow_mut().take() {
                    if let Some(m) = drag.finish() {
                        state.borrow_mut().apply_move(m);
                        *move_count.borrow_mut() += 1;
                    }
                    // Always update mesh after drag ends
                    *needs_mesh_update.borrow_mut() = true;
                }
                mouse.borrow_mut().end_drag();
            }) as Box<dyn FnMut(MouseEvent)>);

            canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            self.closures.push(closure);
        }

        // Mouse leave - cancel face drag, end camera drag
        {
            let mouse = self.mouse.clone();
            let face_drag = self.face_drag.clone();

            let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                *face_drag.borrow_mut() = None;
                mouse.borrow_mut().end_drag();
            }) as Box<dyn FnMut(MouseEvent)>);

            canvas
                .add_event_listener_with_callback("mouseleave", closure.as_ref().unchecked_ref())?;
            self.closures.push(closure);
        }

        Ok(())
    }

    pub fn render(&mut self) {
        let window = web_sys::window().unwrap();
        let perf = window.performance().unwrap();
        let now = perf.now();

        // Check if mesh needs update (after drag finished)
        if *self.needs_mesh_update.borrow() {
            self.renderer.update_mesh(&self.state.borrow());
            *self.needs_mesh_update.borrow_mut() = false;
        }

        // Update animation
        let mut should_apply_move: Option<Move> = None;
        let mut current_animation: Option<LayerAnimation> = None;
        {
            let mut anim = self.animation.borrow_mut();
            if let Some(ref mut animation) = *anim {
                if animation.update(now) {
                    // Animation complete, apply the move
                    should_apply_move = Some(animation.cube_move);
                    *anim = None;
                } else {
                    // Animation in progress - clone for rendering
                    current_animation = Some(LayerAnimation::new(
                        animation.cube_move,
                        animation.start_time,
                    ));
                    current_animation.as_mut().unwrap().current_angle = animation.current_angle;
                }
            }
        }

        if let Some(m) = should_apply_move {
            self.state.borrow_mut().apply_move(m);
            self.renderer.update_mesh(&self.state.borrow());
        }

        // Render
        let camera = self.camera.borrow();

        // Check if we're doing a face drag
        if let Some(ref drag) = *self.face_drag.borrow() {
            self.renderer
                .render_with_drag(&camera, &self.state.borrow(), drag);
        } else if let Some(ref anim) = current_animation {
            self.renderer
                .render_animated(&camera, &self.state.borrow(), anim);
        } else {
            let model = Mat4::identity();
            self.renderer.render(&camera, &model);
        }
    }

    pub fn scramble(&mut self, notation: &str) {
        let moves = parse_scramble(notation);
        for m in moves {
            self.state.borrow_mut().apply_move(m);
        }
        self.renderer.update_mesh(&self.state.borrow());
    }

    pub fn reset(&mut self) {
        *self.state.borrow_mut() = CubeState::solved();
        *self.move_count.borrow_mut() = 0;
        self.renderer.update_mesh(&self.state.borrow());
    }

    pub fn is_solved(&self) -> bool {
        self.state.borrow().is_solved()
    }

    pub fn get_move_count(&self) -> u32 {
        *self.move_count.borrow()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.camera
            .borrow_mut()
            .set_aspect(width as f32 / height as f32);
        self.renderer.resize(width, height);
    }
}
