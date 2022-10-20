use std::rc::Rc;

use crate::{
    console_log, log,
    rdp::{ImageData, ImageType, MouseEventType, Rdp},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{
    CanvasRenderingContext2d, HtmlButtonElement, HtmlCanvasElement, KeyboardEvent, MouseEvent,
};
struct Canvas {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl Canvas {
    fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("rdp-canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        Self { canvas, ctx }
    }

    fn set_resolution(&self, width: u32, height: u32) {
        // set hight & width
        self.canvas.set_height(height);
        self.canvas.set_width(width);
        self.ctx.rect(0_f64, 0_f64, width as f64, height as f64);
        self.ctx.fill();
    }

    fn bind(&self, rdp: &Rdp) {
        let handler = rdp.clone();
        let key_down = move |e: KeyboardEvent| {
            e.prevent_default();
            e.stop_propagation();
            handler.key_press(e, true);
        };

        let handler = Box::new(key_down) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        self.canvas
            .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let handler = rdp.clone();
        let key_up = move |e: KeyboardEvent| {
            e.prevent_default();
            e.stop_propagation();
            handler.key_press(e, false);
        };

        let handler = Box::new(key_up) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        self.canvas
            .add_event_listener_with_callback("keyup", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let handler = rdp.clone();
        let ctrl_alt_del_btn = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("ctrlaltdel")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .map_err(|_| ())
            .unwrap();
        let ctrl_alt_del = move || {
            handler.ctrl_alt_del();
        };
        let handler = Box::new(ctrl_alt_del) as Box<dyn FnMut()>;

        let cb = Closure::wrap(handler);

        ctrl_alt_del_btn.set_onclick(Some(cb.as_ref().unchecked_ref()));
        cb.forget();

        // On a conventional mouse, buttons 1, 2, and 3 correspond to the left,
        // middle, and right buttons on the mouse.  On a wheel mouse, each step
        // of the wheel upwards is represented by a press and release of button
        // 4, and each step downwards is represented by a press and release of
        // button 5.

        // to do:
        // calculate relation position
        let handler = rdp.clone();
        let mouse_move = move |e: MouseEvent| {
            e.stop_propagation();
            handler.mouse_event(e, MouseEventType::Move);
        };

        let handler = Box::new(mouse_move) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        self.canvas
            .add_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let handler = rdp.clone();
        let mouse_down = move |e: MouseEvent| {
            e.stop_propagation();
            handler.mouse_event(e, MouseEventType::Down);
        };

        let handler = Box::new(mouse_down) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        self.canvas
            .add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let handler = rdp.clone();
        let mouse_up = move |e: MouseEvent| {
            e.stop_propagation();
            handler.mouse_event(e, MouseEventType::Up);
        };

        let handler = Box::new(mouse_up) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        self.canvas
            .add_event_listener_with_callback("mouseup", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let get_context_menu = move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
        };

        let handler = Box::new(get_context_menu) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        self.canvas
            .add_event_listener_with_callback("contextmenu", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
    }

    fn draw(&self, ri: &ImageData) {
        match ri.type_ {
            ImageType::Copy => {
                //copy
                let sx = (ri.data[0] as u16) << 8 | ri.data[1] as u16;
                let sy = (ri.data[2] as u16) << 8 | ri.data[3] as u16;

                let _ = self
                    .ctx
                    .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        &self.canvas,
                        sx as f64,
                        sy as f64,
                        ri.width as f64,
                        ri.height as f64,
                        ri.x as f64,
                        ri.y as f64,
                        ri.width as f64,
                        ri.height as f64,
                    );
            }
            ImageType::Fill => {
                // fill
                let (r, g, b) = (ri.data[2], ri.data[1], ri.data[0]);
                let style = format!("rgb({},{},{})", r, g, b);
                self.ctx.set_fill_style(&JsValue::from_str(&style));
            }
            ImageType::Raw => {
                let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
                    Clamped(&ri.data),
                    ri.width as u32,
                    ri.height as u32,
                );
                if data.is_err() {
                    console_log!(
                        "renderring failed at ({}, {}), width {}, height {}, len {}",
                        ri.x,
                        ri.y,
                        ri.width,
                        ri.height,
                        ri.data.len(),
                    );
                }
                let data = data.unwrap();
                let _ = self.ctx.put_image_data(&data, ri.x as f64, ri.y as f64);
            }
        }
    }

    fn close(&self) {
        self.ctx.fill();
    }
}

pub struct CanvasUtils {
    inner: Rc<Canvas>,
}

impl Clone for CanvasUtils {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl CanvasUtils {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(Canvas::new()),
        }
    }

    pub fn init(&self, width: u32, height: u32) {
        self.inner.as_ref().set_resolution(width, height);
    }

    pub fn bind(&self, rdp: &Rdp) {
        self.inner.as_ref().bind(rdp);
    }

    pub fn draw(&self, ri: &ImageData) {
        self.inner.as_ref().draw(ri);
    }

    pub fn close(&self) {
        self.inner.as_ref().close()
    }
}
