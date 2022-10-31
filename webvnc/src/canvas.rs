// use crate::input::{X11Event, KeyEventType, MouseEventType};
// use rdp::core::event::BitmapEvent;
use crate::{
    x11cursor::MouseUtils,
    x11keyboard::{self, KeyboardUtils},
};

use std::rc::Rc;
use tokio::sync::mpsc;
use vnc::{Rect, X11Event};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{
    CanvasRenderingContext2d, HtmlButtonElement, HtmlCanvasElement, HtmlImageElement,
    KeyboardEvent, MouseEvent,
};

struct Canvas {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    output: mpsc::Sender<X11Event>,
}

impl Canvas {
    fn new(sender: mpsc::Sender<X11Event>) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("vnc-canvas").unwrap();
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
        Self {
            canvas,
            ctx,
            output: sender,
        }
    }

    fn set_resolution(&self, width: u32, height: u32) {
        // set hight & width
        self.canvas.set_height(height);
        self.canvas.set_width(width);
        self.ctx.rect(0_f64, 0_f64, width as f64, height as f64);
        self.ctx.fill();
    }

    fn bind(&self) {
        let sender = self.output.clone();
        let key_down = move |e: KeyboardEvent| {
            let sender = sender.clone();
            e.prevent_default();
            e.stop_propagation();
            futures::executor::block_on(async move {
                let _ = sender
                    .send(X11Event::KeyEvent(
                        (KeyboardUtils::get_keysym(e), true).into(),
                    ))
                    .await;
            });
        };

        let handler = Box::new(key_down) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        self.canvas
            .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let sender = self.output.clone();
        let key_up = move |e: KeyboardEvent| {
            let sender = sender.clone();
            e.prevent_default();
            e.stop_propagation();
            futures::executor::block_on(async move {
                let _ = sender
                    .send(X11Event::KeyEvent(
                        (KeyboardUtils::get_keysym(e), false).into(),
                    ))
                    .await;
            });
        };

        let handler = Box::new(key_up) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        self.canvas
            .add_event_listener_with_callback("keyup", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let sender = self.output.clone();
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
            let sender = sender.clone();
            futures::executor::block_on(async move {
                let _ = sender
                    .send(X11Event::KeyEvent((x11keyboard::XK_Control_L, true).into()))
                    .await;
                let _ = sender
                    .send(X11Event::KeyEvent((x11keyboard::XK_Alt_L, true).into()))
                    .await;
                let _ = sender
                    .send(X11Event::KeyEvent((x11keyboard::XK_Delete, true).into()))
                    .await;
                let _ = sender
                    .send(X11Event::KeyEvent((x11keyboard::XK_Delete, false).into()))
                    .await;
                let _ = sender
                    .send(X11Event::KeyEvent((x11keyboard::XK_Alt_L, false).into()))
                    .await;
                let _ = sender
                    .send(X11Event::KeyEvent(
                        (x11keyboard::XK_Control_L, false).into(),
                    ))
                    .await;
            });
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
        let sender = self.output.clone();
        let mouse_move = move |e: MouseEvent| {
            let sender = sender.clone();
            e.prevent_default();
            e.stop_propagation();
            let (x, y, mask) = MouseUtils::get_mouse_sym(e);
            futures::executor::block_on(async move {
                let _ = sender
                    .send(X11Event::PointerEvent((x, y, mask).into()))
                    .await;
            });
        };

        let handler = Box::new(mouse_move) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        self.canvas
            .add_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let sender = self.output.clone();
        let mouse_down = move |e: MouseEvent| {
            let sender = sender.clone();
            // e.prevent_default();
            e.stop_propagation();
            let (x, y, mask) = MouseUtils::get_mouse_sym(e);
            futures::executor::block_on(async move {
                let _ = sender
                    .send(X11Event::PointerEvent((x, y, mask).into()))
                    .await;
            });
        };

        let handler = Box::new(mouse_down) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        self.canvas
            .add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let sender = self.output.clone();
        let mouse_up = move |e: MouseEvent| {
            let sender = sender.clone();
            e.prevent_default();
            e.stop_propagation();
            let (x, y, mask) = MouseUtils::get_mouse_sym(e);
            futures::executor::block_on(async move {
                let _ = sender
                    .send(X11Event::PointerEvent((x, y, mask).into()))
                    .await;
            });
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

    fn draw(&self, rect: Rect, data: Vec<u8>) {
        let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&data),
            rect.width as u32,
            rect.height as u32,
        );

        let data = data.unwrap();
        let _ = self.ctx.put_image_data(&data, rect.x as f64, rect.y as f64);
    }

    fn copy(&self, dst: Rect, src: Rect) {
        //copy
        let _ = self
            .ctx
            .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &self.canvas,
                src.x as f64,
                src.y as f64,
                dst.width as f64,
                dst.height as f64,
                dst.x as f64,
                dst.y as f64,
                dst.width as f64,
                dst.height as f64,
            );
    }

    fn jpeg(&self, rect: Rect, data: Vec<u8>) {
        let image = HtmlImageElement::new().unwrap();
        let base64 = crate::utils::base64_encode(&data);
        image.set_src(&format!(
            "data:image/jpeg;base64,{}",
            std::str::from_utf8(&base64).unwrap()
        ));
        let _ = self.ctx.draw_image_with_html_image_element_and_dw_and_dh(
            &image,
            rect.x as f64,
            rect.y as f64,
            rect.width as f64,
            rect.height as f64,
        );
    }

    fn close(&self) {
        self.ctx.fill();
    }
}

pub struct CanvasUtils {
    inner: Rc<Canvas>,
    bind: bool,
}

impl Clone for CanvasUtils {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            bind: self.bind,
        }
    }
}

impl CanvasUtils {
    pub fn new(sender: mpsc::Sender<X11Event>) -> Self {
        Self {
            inner: Rc::new(Canvas::new(sender)),
            bind: false,
        }
    }

    pub fn init(&mut self, width: u32, height: u32) {
        self.inner.as_ref().set_resolution(width, height);
        if !self.bind {
            self.inner.as_ref().bind();
            self.bind = true;
        }
    }

    pub fn draw(&self, rect: Rect, data: Vec<u8>) {
        self.inner.as_ref().draw(rect, data);
    }

    pub fn copy(&self, dst: Rect, src: Rect) {
        self.inner.as_ref().copy(dst, src);
    }

    pub fn jpeg(&self, rect: Rect, data: Vec<u8>) {
        self.inner.as_ref().jpeg(rect, data);
    }

    pub fn close(&self) {
        self.inner.as_ref().close()
    }
}
