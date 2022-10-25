use crate::input::{InputEvent, KeyEventType, MouseEventType};
use rdp::core::event::BitmapEvent;
use std::rc::Rc;
use tokio::sync::mpsc;
use tracing::warn;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{
    CanvasRenderingContext2d, HtmlButtonElement, HtmlCanvasElement, KeyboardEvent, MouseEvent,
};

struct Canvas {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    output: mpsc::Sender<InputEvent>,
}

impl Canvas {
    fn new(sender: mpsc::Sender<InputEvent>) -> Self {
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
                    .send(InputEvent::Keyboard(e, KeyEventType::Down))
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
                let _ = sender.send(InputEvent::Keyboard(e, KeyEventType::Up)).await;
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
                    .send(InputEvent::KeyCode(
                        0x001D, /* Control Left */
                        KeyEventType::Down,
                    ))
                    .await;
                let _ = sender
                    .send(InputEvent::KeyCode(
                        0x0038, /* Alt Left */
                        KeyEventType::Down,
                    ))
                    .await;
                let _ = sender
                    .send(InputEvent::KeyCode(
                        0xE053, /* Delete */
                        KeyEventType::Down,
                    ))
                    .await;
                let _ = sender
                    .send(InputEvent::KeyCode(
                        0xE053, /* Delete */
                        KeyEventType::Up,
                    ))
                    .await;
                let _ = sender
                    .send(InputEvent::KeyCode(
                        0x0038, /* Alt Left */
                        KeyEventType::Up,
                    ))
                    .await;
                let _ = sender
                    .send(InputEvent::KeyCode(
                        0x001D, /* Control Left */
                        KeyEventType::Up,
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
            futures::executor::block_on(async move {
                let _ = sender
                    .send(InputEvent::Mouse(e, MouseEventType::Move))
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
            futures::executor::block_on(async move {
                let _ = sender
                    .send(InputEvent::Mouse(e, MouseEventType::Down))
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
            futures::executor::block_on(async move {
                let _ = sender.send(InputEvent::Mouse(e, MouseEventType::Up)).await;
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

    fn draw(&self, bm: BitmapEvent) {
        let bitmap_dest_left = bm.dest_left as u32;
        let _bitmap_dest_right = bm.dest_right as u32;
        let _bitmap_dest_bottom = bm.dest_bottom as u32;
        let bitmap_dest_top = bm.dest_top as u32;
        let bitmap_width = bm.width as u32;
        let bitmap_height = bm.height as u32;

        let mut data = bm.decompress().unwrap();
        let mut y = 0;
        let mut x = 0;

        while y < bitmap_height {
            while x < bitmap_width {
                let idx = (y as usize * bitmap_width as usize + x as usize) * 4;
                data.swap(idx, idx + 2);
                data[idx + 3] = 255;
                x += 1;
            }
            x = 0;
            y += 1;
        }

        let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&data),
            bitmap_width as u32,
            bitmap_height as u32,
        );
        if data.is_err() {
            warn!(
                "renderring failed at ({}, {}), width {}, height {}",
                bitmap_dest_left, bitmap_dest_top, bitmap_width, bitmap_height,
            );
        } else {
            //
            // trace!(
            //     "draw x:{}-{}, y:{}-{}",
            //     bitmap_dest_left,
            //     bitmap_dest_right,
            //     bitmap_dest_top,
            //     bitmap_dest_bottom
            // );
        }
        let data = data.unwrap();
        let _ = self
            .ctx
            .put_image_data(&data, bitmap_dest_left as f64, bitmap_dest_top as f64);
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
    pub fn new(sender: mpsc::Sender<InputEvent>) -> Self {
        Self {
            inner: Rc::new(Canvas::new(sender)),
        }
    }

    pub fn init(&self, width: u32, height: u32) {
        self.inner.as_ref().set_resolution(width, height);
        self.inner.as_ref().bind();
    }

    pub fn draw(&self, bm: BitmapEvent) {
        self.inner.as_ref().draw(bm);
    }

    pub fn close(&self) {
        self.inner.as_ref().close()
    }
}
