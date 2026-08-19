use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum EntityType {
    Weapon,
    Ammo,
    Enemy,
    Bullet,
}

pub struct Entity {
    pub tipo: EntityType,
    pub posicion: Vector2,
    pub active: bool,
    pub direccion: Vector2,
    pub vida: f32,
}

impl Entity {
    pub fn weapon(posicion: Vector2) -> Self {
        Self {
            tipo: EntityType::Weapon,
            posicion,
            active: true,
            direccion: Vector2::zero(),
            vida: 0.0,
        }
    }

    pub fn ammo(posicion: Vector2) -> Self {
        Self {
            tipo: EntityType::Ammo,
            posicion,
            active: true,
            direccion: Vector2::zero(),
            vida: 0.0,
        }
    }

    pub fn enemy(posicion: Vector2) -> Self {
        Self {
            tipo: EntityType::Enemy,
            posicion,
            active: true,
            direccion: Vector2::zero(),
            vida: 0.0,
        }
    }

    pub fn bullet(posicion: Vector2, direccion: Vector2) -> Self {
        Self {
            tipo: EntityType::Bullet,
            posicion,
            active: true,
            direccion,
            vida: 1.2,
        }
    }
}
