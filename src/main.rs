mod language_layer;
mod math;
mod win32_engine;

mod entity;
mod entity_manager;

use entity::Entity;
use entity_manager::EntityManager;
use math::{Color, Point, Rect};
use win32_engine::{Win32Drawable, Win32Engine, Win32GameBitmap, Win32Input};

fn main() {
    // Create Win32 window n stuff
    let mut win32_engine = Win32Engine::new("Cheese Game");

    // Win32 xinput (only works for xbox controllers)
    let mut win32_input = Win32Input::new(); // Put inside win32engine?

    // The window buffer
    let mut buffer = Win32GameBitmap::new(win32_engine.get_window());

    let mut entity_manager = EntityManager::new();

    let player = Entity::new(Rect::new(5, 5, 64, 64), entity::EntityType::RECT);

    entity_manager.create(player);

    // let mut _test_read = Win32GameBitmap::load_bmp("Assets/test_file.bmpx");

    while win32_engine.is_running() {
        // Events and input
        win32_engine.handle_events();

        // Always try to get controller
        win32_input.get_controller();

        // Input
        entity_manager.input(&win32_engine, &mut win32_input);

        // Update
        entity_manager.update(&win32_engine);

        // Draw
        win32_engine.clear_screen(0x0FFda025, &mut buffer);

        entity_manager.draw(&win32_engine, &mut buffer);

        win32_engine.render_buffer_to_screen(&mut buffer);
    }

    win32_engine.release(); // Release DC
}
