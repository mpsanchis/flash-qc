use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

#[wasm_bindgen]
pub struct DrawingCanvas {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    is_drawing: Rc<RefCell<bool>>,
    last_x: Rc<RefCell<f64>>,
    last_y: Rc<RefCell<f64>>,
}

#[wasm_bindgen]
impl DrawingCanvas {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<DrawingCanvas, JsValue> {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document");

        let canvas = document
            .get_element_by_id(canvas_id)
            .expect("canvas not found")
            .dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .expect("failed to get 2d context")
            .dyn_into::<CanvasRenderingContext2d>()?;

        // Set up drawing properties
        context.set_line_width(2.0);
        context.set_line_cap("round");
        context.set_line_join("round");
        context.set_stroke_style_str("#000000");

        Ok(DrawingCanvas {
            canvas,
            context,
            is_drawing: Rc::new(RefCell::new(false)),
            last_x: Rc::new(RefCell::new(0.0)),
            last_y: Rc::new(RefCell::new(0.0)),
        })
    }

    pub fn setup_event_listeners(&self) -> Result<(), JsValue> {
        let canvas = self.canvas.clone();

        // Mouse down - start drawing
        {
            let is_drawing = self.is_drawing.clone();
            let last_x = self.last_x.clone();
            let last_y = self.last_y.clone();

            let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                *is_drawing.borrow_mut() = true;
                *last_x.borrow_mut() = event.offset_x() as f64;
                *last_y.borrow_mut() = event.offset_y() as f64;
            }) as Box<dyn FnMut(_)>);

            canvas
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Mouse move - draw
        {
            let context = self.context.clone();
            let is_drawing = self.is_drawing.clone();
            let last_x = self.last_x.clone();
            let last_y = self.last_y.clone();

            let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                if *is_drawing.borrow() {
                    let x = event.offset_x() as f64;
                    let y = event.offset_y() as f64;

                    context.begin_path();
                    context.move_to(*last_x.borrow(), *last_y.borrow());
                    context.line_to(x, y);
                    context.stroke();

                    *last_x.borrow_mut() = x;
                    *last_y.borrow_mut() = y;
                }
            }) as Box<dyn FnMut(_)>);

            canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Mouse up - stop drawing
        {
            let is_drawing = self.is_drawing.clone();

            let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                *is_drawing.borrow_mut() = false;
            }) as Box<dyn FnMut(_)>);

            canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        // Mouse leave - stop drawing
        {
            let is_drawing = self.is_drawing.clone();

            let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
                *is_drawing.borrow_mut() = false;
            }) as Box<dyn FnMut(_)>);

            canvas
                .add_event_listener_with_callback("mouseleave", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        Ok(())
    }
}
