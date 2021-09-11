mod language_layer;
mod math;
mod win32_engine;

use math::{Color, Rect};
use win32_engine::{Win32Drawable, Win32Engine, Win32GameBitmap, Win32Input};

fn main() {
    // Create Win32 window n stuff
    let mut win32_engine = Win32Engine::new("Cheese Game");

    // Win32 xinput (only works for xbox controllers)
    let mut win32_input = Win32Input::new(); // Put inside win32engine?

    // The window buffer
    let mut buffer = Win32GameBitmap::new(win32_engine.get_window());

    let mut player_rect = Rect::new(250, 250, 23, 23);

    while win32_engine.is_running() {
        // Events and input
        win32_engine.handle_events();

        // Always try to get controller
        win32_input.get_controller();

        // Input

        if win32_input.left() {
            player_rect.x -= 1;
        }
        if win32_input.right() {
            player_rect.x += 1;
        }
        if win32_input.up() {
            player_rect.y -= 1;
        }
        if win32_input.down() {
            player_rect.y += 1;
        }

        // Update and Draw
        win32_engine.clear_screen(0x2596be, &mut buffer);

        win32_engine.draw_rectangle(Color::new(0, 0, 1, 1), &mut player_rect, &mut buffer);

        win32_engine.render_buffer_to_screen(&mut buffer);
    }
}
