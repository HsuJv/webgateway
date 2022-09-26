use super::MouseEventType;

pub struct MouseUtils {
    down: bool,
    mask: u8,
}

impl MouseUtils {
    pub fn new() -> Self {
        Self {
            down: false,
            mask: 0,
        }
    }
    pub fn get_mouse_sym(
        &mut self,
        event: web_sys::MouseEvent,
        et: MouseEventType,
    ) -> (u16, u16, u8) {
        let x: u16 = event.offset_x().try_into().unwrap_or(0);
        let y: u16 = event.offset_y().try_into().unwrap_or(0);
        let mask: u8 = (event.button() << 1).try_into().unwrap_or(0);

        match et {
            MouseEventType::MouseDown => {
                self.down = true;
                self.mask = self.down as u8 | mask;
            }
            MouseEventType::MouseUp => {
                self.down = false;
                self.mask = self.down as u8 & (!mask);
            }

            MouseEventType::MouseMove => {}
        }
        (x, y, self.mask)
    }
}
