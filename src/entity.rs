use crate::{
    math::{Color, Rect},
    win32_engine::{Win32Drawable, Win32Engine, Win32GameBitmap, Win32Input},
};

pub enum EntityType {
    RECT,
}

pub struct Entity {
    rect: Rect,
    ent_type: EntityType,
    color: Color,
}

impl Entity {
    pub fn new(rect: Rect, ent_type: EntityType) -> Self {
        Self {
            rect,
            ent_type,
            color: Color::new(0, 0, 0, 0),
        }
    }

    pub fn input(&mut self, engine: &Win32Engine, input: &mut Win32Input) {
        // Only process input if the game window has focus
        if engine.check_focus() {
            // Input
            if input.left() {
                self.rect.x -= 3;
            }
            if input.right() {
                self.rect.x += 3;
            }
            if input.up() {
                self.rect.y -= 3;
            }
            if input.down() {
                self.rect.y += 3;
            }
        }
    }

    pub fn update(&mut self, engine: &Win32Engine) {
        // Screen collision
        if self.rect.x >= (engine.get_width() - self.rect.w) + 1 {
            self.rect.x = engine.get_width() - self.rect.w - 1;
        }
        if self.rect.x <= 0 {
            self.rect.x = 1;
        }
        if self.rect.y >= (engine.get_height() - self.rect.h) + 1 {
            self.rect.y = engine.get_height() - self.rect.h - 1
        }
        if self.rect.y == 0 {
            self.rect.y = 1;
        }
    }

    pub fn draw(&self, engine: &Win32Engine, buffer: &mut Win32GameBitmap) {
        match self.ent_type {
            EntityType::RECT => engine.draw_rectangle(&self.color, &self.rect, buffer),
        }
    }

    pub fn get_type(&self) -> &EntityType {
        &self.ent_type
    }
}
