use super::MouseEventType;

pub struct MouseUtils {
    mask: u8,
}

impl MouseUtils {
    pub fn new() -> Self {
        Self { mask: 0 }
    }
    pub fn get_mouse_sym(
        &mut self,
        event: web_sys::MouseEvent,
        et: MouseEventType,
    ) -> (u16, u16, u8) {
        let x: u16 = event.offset_x().try_into().unwrap_or(0);
        let y: u16 = event.offset_y().try_into().unwrap_or(0);
        let mask: u8 = (1 << event.button()).try_into().unwrap_or(0);

        match et {
            MouseEventType::Down => {
                self.mask |= mask;
            }
            MouseEventType::Up => {
                self.mask &= !mask;
            }

            MouseEventType::Move => {}
        }
        (x, y, self.mask)
    }
}
