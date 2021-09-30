extern crate fps_clock;

mod language_layer;
mod math;
mod win32_engine;

use math::{Color, Point, Rect};
use win32_engine::{Win32Drawable, Win32Engine, Win32GameBitmap, Win32Input};

fn main() {
    // Create Win32 window n stuff
    let mut win32_engine = Win32Engine::new("Cheese Game");

    // Win32 xinput (only works for xbox controllers)
    let mut win32_input = Win32Input::new(); // Put inside win32engine?

    // The window buffer
    let mut buffer = Win32GameBitmap::new(win32_engine.get_window());

    let mut player_rect = Rect::new(250, 250, 64, 64);

    // let mut _test_read = Win32GameBitmap::load_bmp("Assets/test_file.bmpx");

    while win32_engine.is_running() {
        // Events and input
        win32_engine.handle_events();

        // Always try to get controller
        win32_input.get_controller();

        // Only process input if the game window has focus
        if win32_engine.check_focus() {
            // Input
            if win32_input.left() {
                player_rect.x -= 3;
            }
            if win32_input.right() {
                player_rect.x += 3;
            }
            if win32_input.up() {
                player_rect.y -= 3;
            }
            if win32_input.down() {
                player_rect.y += 3;
            }
        }

        // Screen collision
        if player_rect.x >= (win32_engine.get_width() - player_rect.w) + 1 {
            player_rect.x = win32_engine.get_width() - player_rect.w - 1;
        }
        if player_rect.x <= 0 {
            player_rect.x = 1;
        }
        if player_rect.y >= (win32_engine.get_height() - player_rect.h) + 1 {
            player_rect.y = win32_engine.get_height() - player_rect.h - 1
        }
        if player_rect.y <= 0 {
            player_rect.y = 1;
        }

        // Update and Draw
        win32_engine.clear_screen(0x0, &mut buffer);

        //_test_read.draw_bmp(Point::new(25, 25), &mut buffer);

        win32_engine.draw_rectangle(
            Color::new(1, 0, 1, 1),
            &mut Rect::new(450, 450, 64, 64),
            &mut buffer,
        );

        win32_engine.draw_rectangle(Color::new(0, 0, 1, 1), &mut player_rect, &mut buffer);

        win32_engine.render_buffer_to_screen(&mut buffer);
    }

    win32_engine.release(); // Release DC
}
