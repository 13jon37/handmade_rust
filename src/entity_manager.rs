use crate::{
    entity::{self, Entity},
    win32_engine::{Win32Engine, Win32GameBitmap, Win32Input},
};

pub struct EntityManager {
    entities: Vec<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn create(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn input(&mut self, engine: &Win32Engine, input: &mut Win32Input) {
        for entity in &mut self.entities {
            // Only allow input depending on the type
            match entity.get_type() {
                entity::EntityType::RECT => entity.input(engine, input),
            }
        }
    }

    pub fn update(&mut self, engine: &Win32Engine) {
        for entity in &mut self.entities {
            entity.update(engine);
        }
    }

    pub fn draw(&self, engine: &Win32Engine, buffer: &mut Win32GameBitmap) {
        for entity in &self.entities {
            entity.draw(engine, buffer);
        }
    }
}
