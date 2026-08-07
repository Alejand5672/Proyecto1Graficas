use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum EntityType {
    Weapon,
}

pub struct Entity {
    pub tipo: EntityType,
    pub posicion: Vector2,
    pub active: bool,
}

impl Entity {
    pub fn weapon(posicion: Vector2) -> Self {
        Self {
            tipo: EntityType::Weapon,
            posicion,
            active: true,
        }
    }
}
