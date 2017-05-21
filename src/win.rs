extern crate winapi;
extern crate user32;


use self::user32::*;
use self::winapi::*;

use super::{KeyboardControllable, MouseControllable};
use std::mem::*;

/// The main struct for handling the event emitting
pub struct Enigo {
    current_x: i32,
    current_y: i32,
}

impl Enigo {
    // TODO(dustin): to the right initialisation

    /// Constructs a new `Enigo` instance.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use enigo::*;
    /// let mut enigo = Enigo::new();
    /// ```
    pub fn new() -> Self {
        Enigo {
            current_x: 0,
            current_y: 0,
        }
    }
}

impl MouseControllable for Enigo {
    fn mouse_move_to(&mut self, x: i32, y: i32) {
        // TODO(dustin): use interior mutability
        self.current_x = x;
        self.current_y = y;
        unsafe { SetCursorPos(self.current_x, self.current_y) };
    }

    fn mouse_move_relative(&mut self, x: i32, y: i32) {
        // TODO(dustin): use interior mutability
        self.current_x += x;
        self.current_y += y;
        unsafe { SetCursorPos(self.current_x, self.current_y) };
    }

    // TODO(dustin): use button parameter, current implementation
    // is using the left mouse button every time
    fn mouse_down(&mut self, button: u32) {
        unsafe {
            let mut input = INPUT {
                type_: INPUT_MOUSE,
                u: transmute_copy(&MOUSEINPUT {
                                       dx: 0,
                                       dy: 0,
                                       mouseData: 0,
                                       dwFlags: MOUSEEVENTF_LEFTDOWN,
                                       time: 0,
                                       dwExtraInfo: 0,
                                   }),
            };

            SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int);
        }
    }

    // TODO(dustin): use button parameter, current implementation
    // is using the left mouse button every time
    fn mouse_up(&mut self, button: u32) {
        unsafe {
            let mut input = INPUT {
                type_: INPUT_MOUSE,
                u: transmute_copy(&MOUSEINPUT {
                                       dx: 0,
                                       dy: 0,
                                       mouseData: 0,
                                       dwFlags: MOUSEEVENTF_LEFTUP,
                                       time: 0,
                                       dwExtraInfo: 0,
                                   }),
            };

            SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int);
        }
    }

    fn mouse_click(&mut self, button: u32) {
        self.mouse_down(button);
        self.mouse_up(button);
    }

    fn mouse_scroll_x(&mut self, length: i32) {
        let mut scroll_direction = 1 * 50; // 1 left -1 right
        let mut length = length;

        if length < 0 {
            length *= -1;
            scroll_direction *= -1;
        }

        for _ in 0..length {
            unsafe {
                let mut input = INPUT {
                    type_: INPUT_MOUSE,
                    u: transmute_copy(&MOUSEINPUT {
                                           dx: 0,
                                           dy: 0,
                                           mouseData: transmute_copy(&scroll_direction),
                                           dwFlags: MOUSEEVENTF_HWHEEL,
                                           time: 0,
                                           dwExtraInfo: 0,
                                       }),
                };

                SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int);
            }
        }
    }

    fn mouse_scroll_y(&mut self, length: i32) {
        let mut scroll_direction = -1 * 50; // 1 left -1 right
        let mut length = length;

        if length < 0 {
            length *= -1;
            scroll_direction *= -1;
        }

        for _ in 0..length {
            unsafe {
                let mut input = INPUT {
                    type_: INPUT_MOUSE,
                    u: transmute_copy(&MOUSEINPUT {
                                           dx: 0,
                                           dy: 0,
                                           mouseData: transmute_copy(&scroll_direction),
                                           dwFlags: MOUSEEVENTF_WHEEL,
                                           time: 0,
                                           dwExtraInfo: 0,
                                       }),
                };

                SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int);
            }
        }
    }
}

impl KeyboardControllable for Enigo {
    fn key_sequence(&self, sequence: &str) {
        //unimplemented!()
        
        for c in sequence.chars() {
            let mut buffer = [0; 2];
            let result = c.encode_utf16(&mut buffer);
            //Note(dustin): this is wrong utf16 is variable length
            //but i don't know how to feed that properly into SendInput
            let single_u16_result = result[0]; 
            self.keyclick(single_u16_result);
        }       
    }
}

impl Enigo {
    fn keyclick(&self, unicode_char: u16) {
        use std::{thread, time};
        thread::sleep(time::Duration::from_millis(20));
        self.keydown(unicode_char);
        self.keyup(unicode_char); 
        thread::sleep(time::Duration::from_millis(20));
    }

    fn keydown(&self, unicode_char: u16) {
        unsafe {
            let mut input = INPUT {
                type_: INPUT_KEYBOARD,
                u: transmute_copy(&KEYBDINPUT {
                    wVk: 0,
                    wScan: unicode_char,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: 0,
                }),
            };
        
            SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int);
        }
    }

    fn keyup(&self, unicode_char: u16) {
        unsafe {
            let mut input = INPUT {
                type_: INPUT_MOUSE,
                u: transmute_copy(&KEYBDINPUT {
                    wVk: 0,
                    wScan: unicode_char,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                }),
            };
        
            SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int);
        }
    }
}
