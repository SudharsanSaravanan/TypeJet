use pyo3::prelude::*;
use rand::RngExt;

struct TypingDelays {
    min_dwell: u64,
    max_dwell: u64,
    min_shift: u64,
    max_shift: u64,
}

// Safely generate an integer delay (prevents panic if min == max or if min > max)
fn get_delay(min: u64, max: u64) -> u64 {
    let low = min.min(max);
    let high = min.max(max);
    if high == 0 {
        0
    } else if low == high {
        low
    } else {
        rand::rng().random_range(low..=high)
    }
}

// Safely generate a float delay
fn get_char_delay(min: f64, max: f64) -> f64 {
    let low = min.min(max);
    let high = min.max(max);
    if high <= 0.0 {
        0.0
    } else if low == high {
        low
    } else {
        rand::rng().random_range(low..=high)
    }
}

#[cfg(target_os = "linux")]
mod linux_impl {
    use super::{get_char_delay, get_delay, TypingDelays};
    use input_linux::{EventKind, Key, UInputHandle};
    use std::thread::sleep;
    use std::time::Duration;

    fn write_event(
        ui: &UInputHandle<std::fs::File>,
        type_: u16,
        code: u16,
        value: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let raw = input_linux::sys::input_event {
            time: input_linux::sys::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            type_,
            code,
            value,
        };
        ui.write(&[raw])?;
        Ok(())
    }

    fn write_sync_event(ui: &UInputHandle<std::fs::File>) -> Result<(), Box<dyn std::error::Error>> {
        write_event(
            ui,
            input_linux::sys::EV_SYN as u16,
            input_linux::sys::SYN_REPORT as u16,
            0,
        )
    }

    fn send_key(
        ui: &UInputHandle<std::fs::File>,
        key: Key,
        delays: &TypingDelays,
    ) -> Result<(), Box<dyn std::error::Error>> {
        write_event(ui, input_linux::sys::EV_KEY as u16, key.code(), 1)?;
        write_sync_event(ui)?;

        let dwell_time = get_delay(delays.min_dwell, delays.max_dwell);
        if dwell_time > 0 {
            sleep(Duration::from_millis(dwell_time));
        }

        write_event(ui, input_linux::sys::EV_KEY as u16, key.code(), 0)?;
        write_sync_event(ui)?;

        Ok(())
    }

    fn send_shifted_key(
        ui: &UInputHandle<std::fs::File>,
        key: Key,
        delays: &TypingDelays,
    ) -> Result<(), Box<dyn std::error::Error>> {
        write_event(ui, input_linux::sys::EV_KEY as u16, Key::LeftShift.code(), 1)?;
        write_sync_event(ui)?;

        let shift_delay = get_delay(delays.min_shift, delays.max_shift);
        if shift_delay > 0 {
            sleep(Duration::from_millis(shift_delay));
        }

        write_event(ui, input_linux::sys::EV_KEY as u16, key.code(), 1)?;
        write_sync_event(ui)?;

        let dwell_time = get_delay(delays.min_dwell, delays.max_dwell);
        if dwell_time > 0 {
            sleep(Duration::from_millis(dwell_time));
        }

        write_event(ui, input_linux::sys::EV_KEY as u16, key.code(), 0)?;
        write_sync_event(ui)?;

        let unshift_delay = get_delay(delays.min_shift, delays.max_shift);
        if unshift_delay > 0 {
            sleep(Duration::from_millis(unshift_delay));
        }

        write_event(ui, input_linux::sys::EV_KEY as u16, Key::LeftShift.code(), 0)?;
        write_sync_event(ui)?;

        Ok(())
    }

    fn char_to_key(c: char) -> Option<(Key, bool)> {
        use Key::*;
        Some(match c {
            'a' => (A, false),
            'b' => (B, false),
            'c' => (C, false),
            'd' => (D, false),
            'e' => (E, false),
            'f' => (F, false),
            'g' => (G, false),
            'h' => (H, false),
            'i' => (I, false),
            'j' => (J, false),
            'k' => (K, false),
            'l' => (L, false),
            'm' => (M, false),
            'n' => (N, false),
            'o' => (O, false),
            'p' => (P, false),
            'q' => (Q, false),
            'r' => (R, false),
            's' => (S, false),
            't' => (T, false),
            'u' => (U, false),
            'v' => (V, false),
            'w' => (W, false),
            'x' => (X, false),
            'y' => (Y, false),
            'z' => (Z, false),

            'A' => (A, true),
            'B' => (B, true),
            'C' => (C, true),
            'D' => (D, true),
            'E' => (E, true),
            'F' => (F, true),
            'G' => (G, true),
            'H' => (H, true),
            'I' => (I, true),
            'J' => (J, true),
            'K' => (K, true),
            'L' => (L, true),
            'M' => (M, true),
            'N' => (N, true),
            'O' => (O, true),
            'P' => (P, true),
            'Q' => (Q, true),
            'R' => (R, true),
            'S' => (S, true),
            'T' => (T, true),
            'U' => (U, true),
            'V' => (V, true),
            'W' => (W, true),
            'X' => (X, true),
            'Y' => (Y, true),
            'Z' => (Z, true),

            '0' => (Num0, false),
            '1' => (Num1, false),
            '2' => (Num2, false),
            '3' => (Num3, false),
            '4' => (Num4, false),
            '5' => (Num5, false),
            '6' => (Num6, false),
            '7' => (Num7, false),
            '8' => (Num8, false),
            '9' => (Num9, false),

            '!' => (Num1, true),
            '@' => (Num2, true),
            '#' => (Num3, true),
            '$' => (Num4, true),
            '%' => (Num5, true),
            '^' => (Num6, true),
            '&' => (Num7, true),
            '*' => (Num8, true),
            '(' => (Num9, true),
            ')' => (Num0, true),

            ' ' => (Space, false),
            '-' => (Minus, false),
            '_' => (Minus, true),
            '=' => (Equal, false),
            '+' => (Equal, true),

            '[' => (LeftBrace, false),
            '{' => (LeftBrace, true),
            ']' => (RightBrace, false),
            '}' => (RightBrace, true),

            ';' => (Semicolon, false),
            ':' => (Semicolon, true),
            '\'' => (Apostrophe, false),
            '"' => (Apostrophe, true),

            ',' => (Comma, false),
            '<' => (Comma, true),
            '.' => (Dot, false),
            '>' => (Dot, true),
            '/' => (Slash, false),
            '?' => (Slash, true),

            '\\' => (Backslash, false),
            '|' => (Backslash, true),
            '`' => (Grave, false),
            '~' => (Grave, true),

            '\n' => (Enter, false),

            _ => return None,
        })
    }

    pub fn type_text_os(
        text: &str,
        min_char_delay: f64,
        max_char_delay: f64,
        delays: &TypingDelays,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/uinput")?;

        let ui = UInputHandle::new(file);

        ui.set_evbit(EventKind::Key)?;

        for key in Key::iter() {
            let _ = ui.set_keybit(key);
        }

        let input_id_raw = input_linux::sys::input_id {
            bustype: input_linux::sys::BUS_USB as u16,
            vendor: 0x1234,
            product: 0x5678,
            version: 0,
        };

        let input_id: input_linux::InputId = unsafe { std::mem::transmute(input_id_raw) };

        ui.create(&input_id, b"rs_uinput_kb\0", 0, &[])?;

        // The OS needs a fraction of a second to register the virtual device
        sleep(Duration::from_millis(500));

        for ch in text.chars() {
            if let Some((key, shifted)) = char_to_key(ch) {
                if shifted {
                    send_shifted_key(&ui, key, delays)?;
                } else {
                    send_key(&ui, key, delays)?;
                }

                let char_delay = get_char_delay(min_char_delay, max_char_delay);
                if char_delay > 0.0 {
                    sleep(Duration::from_secs_f64(char_delay));
                }
            }
        }

        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod windows_impl {
    use super::{get_char_delay, get_delay, TypingDelays};
    use std::mem::size_of;
    use std::thread::sleep;
    use std::time::Duration;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput,
    };

    const VK_SHIFT: u16 = 0x10;
    const VK_SPACE: u16 = 0x20;
    const VK_RETURN: u16 = 0x0D;

    fn send_key_windows(vk: u16, down: bool) {
        let mut input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: unsafe { std::mem::zeroed() },
        };

        let flags = if down { 0 } else { KEYEVENTF_KEYUP };

        input.Anonymous.ki = KEYBDINPUT {
            wVk: vk,
            wScan: 0,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: 0,
        };

        unsafe {
            SendInput(1, &input as *const INPUT, size_of::<INPUT>() as i32);
        }
    }

    fn send_key(vk: u16, delays: &TypingDelays) {
        send_key_windows(vk, true);

        let dwell_time = get_delay(delays.min_dwell, delays.max_dwell);
        if dwell_time > 0 {
            sleep(Duration::from_millis(dwell_time));
        }

        send_key_windows(vk, false);
    }

    fn send_shifted_key(vk: u16, delays: &TypingDelays) {
        send_key_windows(VK_SHIFT, true);

        let shift_delay = get_delay(delays.min_shift, delays.max_shift);
        if shift_delay > 0 {
            sleep(Duration::from_millis(shift_delay));
        }

        send_key_windows(vk, true);

        let dwell_time = get_delay(delays.min_dwell, delays.max_dwell);
        if dwell_time > 0 {
            sleep(Duration::from_millis(dwell_time));
        }

        send_key_windows(vk, false);

        let unshift_delay = get_delay(delays.min_shift, delays.max_shift);
        if unshift_delay > 0 {
            sleep(Duration::from_millis(unshift_delay));
        }

        send_key_windows(VK_SHIFT, false);
    }

    fn char_to_vk(c: char) -> Option<(u16, bool)> {
        Some(match c {
            'a'..='z' => (0x41 + (c as u16 - 'a' as u16), false),
            'A'..='Z' => (0x41 + (c as u16 - 'A' as u16), true),
            '0'..='9' => (0x30 + (c as u16 - '0' as u16), false),

            '!' => (0x31, true),
            '@' => (0x32, true),
            '#' => (0x33, true),
            '$' => (0x34, true),
            '%' => (0x35, true),
            '^' => (0x36, true),
            '&' => (0x37, true),
            '*' => (0x38, true),
            '(' => (0x39, true),
            ')' => (0x30, true),

            ' ' => (VK_SPACE, false),
            '-' => (0xBD, false),
            '_' => (0xBD, true),
            '=' => (0xBB, false),
            '+' => (0xBB, true),

            '[' => (0xDB, false),
            '{' => (0xDB, true),
            ']' => (0xDD, false),
            '}' => (0xDD, true),

            ';' => (0xBA, false),
            ':' => (0xBA, true),
            '\'' => (0xDE, false),
            '"' => (0xDE, true),

            ',' => (0xBC, false),
            '<' => (0xBC, true),
            '.' => (0xBE, false),
            '>' => (0xBE, true),
            '/' => (0xBF, false),
            '?' => (0xBF, true),

            '\\' => (0xDC, false),
            '|' => (0xDC, true),
            '`' => (0xC0, false),
            '~' => (0xC0, true),

            '\n' => (VK_RETURN, false),

            _ => return None,
        })
    }

    pub fn type_text_os(
        text: &str,
        min_char_delay: f64,
        max_char_delay: f64,
        delays: &TypingDelays,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for ch in text.chars() {
            if let Some((vk, shifted)) = char_to_vk(ch) {
                if shifted {
                    send_shifted_key(vk, delays);
                } else {
                    send_key(vk, delays);
                }

                let char_delay = get_char_delay(min_char_delay, max_char_delay);
                if char_delay > 0.0 {
                    sleep(Duration::from_secs_f64(char_delay));
                }
            }
        }
        Ok(())
    }
}

/// Simulates hardware keystrokes using Linux's /dev/uinput or Windows' SendInput.
///
/// By default, this uses jittered delays to mimic natural human typing dynamics.
/// For instantaneous machine-speed typing, set all delays to 0.
///
/// Args:
///     text (str): The string of characters to type.
///     min_char_delay (float): Minimum delay between keystrokes in seconds. Default 0.05.
///     max_char_delay (float): Maximum delay between keystrokes in seconds. Default 0.1.
///     min_dwell_time (int): Minimum time a key is physically held down in milliseconds. Default 20.
///     max_dwell_time (int): Maximum time a key is physically held down in milliseconds. Default 60.
///     min_shift_delay (int): Minimum hesitation before/after pressing shift in milliseconds. Default 10.
///     max_shift_delay (int): Maximum hesitation before/after pressing shift in milliseconds. Default 30.
#[pyfunction]
#[pyo3(signature = (
    text,
    min_char_delay=0.05,
    max_char_delay=0.1,
    min_dwell_time=20,
    max_dwell_time=60,
    min_shift_delay=10,
    max_shift_delay=30
))]
fn type_text(
    text: String,
    min_char_delay: f64,
    max_char_delay: f64,
    min_dwell_time: u64,
    max_dwell_time: u64,
    min_shift_delay: u64,
    max_shift_delay: u64,
) -> PyResult<()> {
    let delays = TypingDelays {
        min_dwell: min_dwell_time,
        max_dwell: max_dwell_time,
        min_shift: min_shift_delay,
        max_shift: max_shift_delay,
    };

    #[cfg(target_os = "linux")]
    {
        linux_impl::type_text_os(&text, min_char_delay, max_char_delay, &delays)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    }

    #[cfg(target_os = "windows")]
    {
        windows_impl::type_text_os(&text, min_char_delay, max_char_delay, &delays)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    }

    Ok(())
}

#[pymodule]
fn typejet(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(type_text, m)?)?;
    Ok(())
}