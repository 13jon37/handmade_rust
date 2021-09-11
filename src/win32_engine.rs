use std::mem;

use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPCWSTR;
use winapi::shared::windef::{HBRUSH, HICON, HMENU, HWND, RECT};
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::wingdi::{
    StretchDIBits, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, RGBQUAD, SRCCOPY,
};

use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE};
use winapi::um::winuser::*;

use kernel32::*;
use winapi::um::xinput::{
    XInputGetState, XINPUT_GAMEPAD, XINPUT_GAMEPAD_DPAD_DOWN, XINPUT_GAMEPAD_DPAD_LEFT,
    XINPUT_GAMEPAD_DPAD_RIGHT, XINPUT_GAMEPAD_DPAD_UP, XINPUT_STATE, XUSER_MAX_COUNT,
};

use crate::language_layer::create_wide_char;
use crate::math::{Color, Rect};

static mut IS_WINDOW_CLOSED: bool = false;

pub trait Win32Drawable {
    fn draw_rectangle(&self, color: Color, rect: &mut Rect, buffer: &mut Win32GameBitmap);
}

pub enum WindowMessages {
    WindowClosed,
}

// Storage for Screen data that excludes the windows bar
pub struct ClientData {
    width: i32,
    height: i32,
}

pub fn get_client_data(window: &HWND) -> ClientData {
    let mut result = ClientData {
        width: 0,
        height: 0,
    };

    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    unsafe {
        GetClientRect(*window, &mut rect);
    }

    result.width = rect.right - rect.left;
    result.height = rect.bottom - rect.top;

    result
}

pub struct Win32GameBitmap {
    pub bitmap_info: BITMAPINFO,
    pub memory: *const winapi::ctypes::c_void,
}

impl Win32GameBitmap {
    pub fn new(window: &HWND) -> Self {
        // Null init
        let colors = [RGBQUAD {
            rgbBlue: 0,
            rgbGreen: 0,
            rgbRed: 0,
            rgbReserved: 0,
        }];

        let buffer: BITMAPINFO = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: get_client_data(window).width,
                biHeight: -get_client_data(window).height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: colors,
        };

        // Fill the buffer
        let buffer_size = get_client_data(window).width
            * get_client_data(window).height
            * mem::size_of::<u32>() as i32;
        let mut _memory = 0 as *const winapi::ctypes::c_void;

        unsafe {
            if !_memory.is_null() {
                VirtualFree(_memory as *mut std::ffi::c_void, 0, MEM_RELEASE);
            }

            _memory = VirtualAlloc(
                std::ptr::null_mut(),
                buffer_size as u64,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            ) as *const winapi::ctypes::c_void;
        }

        Self {
            bitmap_info: buffer,
            memory: _memory,
        }
    }
}

unsafe extern "system" fn window_proc(
    h_wnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if msg == WM_DESTROY {
        IS_WINDOW_CLOSED = true;

        PostQuitMessage(0);
    }

    DefWindowProcW(h_wnd, msg, w_param, l_param)
}

pub struct Win32Engine {
    running: bool,
    hwnd: HWND,
    screen_data: ClientData,
}

impl Win32Engine {
    pub fn new(window_name: &str) -> Self {
        unsafe {
            let window_class = WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: 0 as HINSTANCE,
                hIcon: 0 as HICON,
                hCursor: 0 as HICON,
                hbrBackground: 16 as HBRUSH,
                lpszMenuName: 0 as LPCWSTR,
                lpszClassName: create_wide_char("MyWindowClass").as_ptr(),
            };

            let error_code = RegisterClassW(&window_class);

            assert!(error_code != 0, "failed to register the window class");

            let window = CreateWindowExW(
                0,
                create_wide_char("MyWindowClass").as_ptr(),
                create_wide_char(window_name).as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                0 as HWND,
                0 as HMENU,
                0 as HINSTANCE,
                std::ptr::null_mut(),
            );

            assert!(window != (0 as HWND), "failed to open the window");

            ShowWindow(window, SW_SHOW);
            UpdateWindow(window);

            Self {
                running: true,
                hwnd: window,
                screen_data: get_client_data(&window),
            }
        }
    }

    pub fn process_window_messages(&self) -> Option<WindowMessages> {
        unsafe {
            let mut msg: MSG = std::mem::zeroed();

            // process messages
            while PeekMessageA(&mut msg, self.hwnd, 0, 0, PM_REMOVE) > 0 {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);

                if IS_WINDOW_CLOSED {
                    return Some(WindowMessages::WindowClosed);
                }
            }

            None
        }
    }

    fn resize_buffer(&self, width: i32, height: i32, buffer: &mut Win32GameBitmap) {
        let buffer_size = width * height * mem::size_of::<u32>() as i32;

        unsafe {
            if !buffer.memory.is_null() {
                VirtualFree(buffer.memory as *mut std::ffi::c_void, 0, MEM_RELEASE);
            }

            let colors = [RGBQUAD {
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0,
            }];

            // Refill the buffer info
            buffer.bitmap_info = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width,
                    biHeight: -height,
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: colors,
            };

            buffer.memory = VirtualAlloc(
                0 as *mut std::ffi::c_void,
                buffer_size as u64,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            ) as *const winapi::ctypes::c_void;
        }
    }

    pub fn clear_screen(&self, color: u32, buffer: &mut Win32GameBitmap) {
        unsafe {
            let pixel = buffer.memory as *mut u32;

            let mut counter = 0;
            for _y in 0..self.screen_data.height {
                for _x in 0..self.screen_data.width {
                    counter += 1;
                    *pixel.add(counter) = color;

                    // Confused me because in C
                    // it's *pixel++ = color;
                }
            }
        }
    }

    pub fn render_buffer_to_screen(&mut self, buffer: &mut Win32GameBitmap) {
        let current_data = get_client_data(&self.hwnd);

        if self.screen_data.width != current_data.width
            || self.screen_data.height != current_data.height
        {
            // Resize the buffer
            self.resize_buffer(current_data.width, current_data.height, buffer);

            // Set new render res
            self.screen_data.width = current_data.width;
            self.screen_data.height = current_data.height;
        }

        unsafe {
            StretchDIBits(
                GetDC(self.hwnd),
                0,
                0,
                self.screen_data.width,
                self.screen_data.height,
                0,
                0,
                self.screen_data.width,
                self.screen_data.height,
                buffer.memory,
                &buffer.bitmap_info,
                DIB_RGB_COLORS,
                SRCCOPY,
            );
        }
    }

    pub fn handle_events(&mut self) {
        while let Some(x) = self.process_window_messages() {
            match x {
                WindowMessages::WindowClosed => {
                    self.running = false;
                }
            }
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn get_window(&self) -> &HWND {
        &self.hwnd
    }

    pub fn get_width(&self) -> u32 {
        self.screen_data.width as u32
    }

    pub fn get_height(&self) -> u32 {
        self.screen_data.height as u32
    }
}

// Win32 Draw functions/methods
impl Win32Drawable for Win32Engine {
    // Thank you Ryan Fleury, I was too stupid to figure this out so I just
    // translated your C code to rust. :) :dumb:
    fn draw_rectangle(&self, color: Color, rect: &mut Rect, buffer: &mut Win32GameBitmap) {
        unsafe {
            let pixel = buffer.memory as *mut u8; // Starting pixel

            let lower_bound_x = rect.x;
            let lower_bound_y = rect.y;
            let upper_bound_x = lower_bound_x + rect.w;
            let upper_bound_y = lower_bound_y + rect.h;

            let mut _pixel_index = 0;
            for y in lower_bound_y..upper_bound_y {
                for x in lower_bound_x..upper_bound_x {
                    _pixel_index = y * self.screen_data.width as u32 + x;
                    // Pixel index is the pixel coord we wanna change the color of
                    *pixel.add((_pixel_index * 4 + 0) as usize) = color.r * 255;
                    *pixel.add((_pixel_index * 4 + 1) as usize) = color.g * 255;
                    *pixel.add((_pixel_index * 4 + 2) as usize) = color.b * 255;
                }
            }
        }
    }
}

pub struct Win32Input {
    game_pad_state: XINPUT_STATE,
    game_pad_id: i8,
}

impl Win32Input {
    pub fn new() -> Self {
        // Null init XINPUT structures
        let gamepad_struct = XINPUT_GAMEPAD {
            wButtons: 0,
            bLeftTrigger: 0,
            bRightTrigger: 0,
            sThumbLX: 0,
            sThumbLY: 0,
            sThumbRX: 0,
            sThumbRY: 0,
        };

        let state = XINPUT_STATE {
            dwPacketNumber: 0,
            Gamepad: gamepad_struct,
        };

        Self {
            game_pad_state: state,
            game_pad_id: -1,
        }
    }

    pub fn get_controller(&mut self) {
        if self.game_pad_id == -1 {
            for i in 0..XUSER_MAX_COUNT {
                let empty_gamepad_struct = XINPUT_GAMEPAD {
                    wButtons: 0,
                    bLeftTrigger: 0,
                    bRightTrigger: 0,
                    sThumbLX: 0,
                    sThumbLY: 0,
                    sThumbRX: 0,
                    sThumbRY: 0,
                };

                let mut state = XINPUT_STATE {
                    dwPacketNumber: 0,
                    Gamepad: empty_gamepad_struct,
                };

                unsafe {
                    if XInputGetState(i, &mut state) == ERROR_SUCCESS {
                        self.game_pad_id = i as i8;
                        println!("Found controller!");
                    }
                }
            }
        }
    }

    pub fn left(&mut self) -> bool {
        unsafe {
            if GetAsyncKeyState(0x41) != 0 || GetAsyncKeyState(VK_LEFT) != 0 {
                return true;
            }

            if XInputGetState(self.game_pad_id as u32, &mut self.game_pad_state) == ERROR_SUCCESS {
                if self.game_pad_state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_LEFT != 0 {
                    return true;
                }
            }
        }

        false
    }

    pub fn right(&mut self) -> bool {
        unsafe {
            if GetAsyncKeyState(0x44) != 0 || GetAsyncKeyState(VK_RIGHT) != 0 {
                return true;
            }

            if self.game_pad_state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_RIGHT != 0 {
                return true;
            }
        }
        false
    }

    pub fn up(&mut self) -> bool {
        unsafe {
            if GetAsyncKeyState(0x57) != 0 || GetAsyncKeyState(VK_UP) != 0 {
                return true;
            }

            if self.game_pad_state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_UP != 0 {
                return true;
            }
        }
        false
    }

    pub fn down(&mut self) -> bool {
        unsafe {
            if GetAsyncKeyState(0x53) != 0 || GetAsyncKeyState(VK_DOWN) != 0 {
                return true;
            }
            if self.game_pad_state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_DOWN != 0 {
                return true;
            }
        }
        false
    }
}
