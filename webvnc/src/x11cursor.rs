pub struct MouseUtils;

impl MouseUtils {
    pub fn get_mouse_sym(event: web_sys::MouseEvent) -> (u16, u16, u8) {
        let x: u16 = event.offset_x().try_into().unwrap_or(0);
        let y: u16 = event.offset_y().try_into().unwrap_or(0);
        let buttons = event.buttons();

        // On a conventional mouse, buttons 1, 2, and 3 correspond to the left,
        // middle, and right buttons on the mouse.  On a wheel mouse, each step
        // of the wheel upwards is represented by a press and release of button
        // 4, and each step downwards is represented by a press and release of
        // button 5.
        let mut mask = 0;
        let left = buttons & 0x1 > 0;
        let right = buttons & 0x2 > 0;
        let middle = buttons & 0x4 > 0;
        if left {
            mask |= 1;
        }
        if middle {
            mask |= 1 << 1;
        }
        if right {
            mask |= 1 << 2;
        }
        (x, y, mask)
    }
}
