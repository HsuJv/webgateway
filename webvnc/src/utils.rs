pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

const BASIS_64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_encode(input: &[u8]) -> Vec<u8> {
    let mut i = 0;
    let len = input.len();
    let mut out = Vec::with_capacity(((len + 2) / 3 * 4) + 1);

    while i < len - 2 {
        out.push(BASIS_64[(input[i] as usize >> 2) & 0x3F]);
        out.push(
            BASIS_64[((input[i] as usize & 0x3) << 4) | ((input[i + 1] as usize & 0xF0) >> 4)],
        );
        out.push(
            BASIS_64[((input[i + 1] as usize & 0xF) << 2) | ((input[i + 2] as usize & 0xC0) >> 6)],
        );
        out.push(BASIS_64[input[i + 2] as usize & 0x3F]);
        i += 3;
    }

    if i < len {
        out.push(BASIS_64[(input[i] as usize >> 2) & 0x3F]);
        if i == (len - 1) {
            out.push(BASIS_64[((input[i] as usize & 0x3) << 4)]);
            out.push(0x3d); // =
        } else {
            out.push(
                BASIS_64[((input[i] as usize & 0x3) << 4) | ((input[i + 1] as usize & 0xF0) >> 4)],
            );
            out.push(BASIS_64[((input[i + 1] as usize & 0xF) << 2)]);
        }
        out.push(0x3d); // =
    }
    out
}
