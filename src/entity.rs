use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum EntityType {
    Weapon,
    Ammo,
    Enemy,
    Bullet,
    EnemyBullet,
}

pub struct Entity {
    pub tipo: EntityType,
    pub posicion: Vector2,
    pub active: bool,
    pub direccion: Vector2,
    pub vida: f32,
    pub cooldown: f32,
}

impl Entity {
    pub fn weapon(posicion: Vector2) -> Self {
        Self {
            tipo: EntityType::Weapon,
            posicion,
            active: true,
            direccion: Vector2::zero(),
            vida: 0.0,
            cooldown: 0.0,
        }
    }

    pub fn ammo(posicion: Vector2) -> Self {
        Self {
            tipo: EntityType::Ammo,
            posicion,
            active: true,
            direccion: Vector2::zero(),
            vida: 0.0,
            cooldown: 0.0,
        }
    }

    pub fn enemy(posicion: Vector2) -> Self {
        Self {
            tipo: EntityType::Enemy,
            posicion,
            active: true,
            direccion: Vector2::zero(),
            vida: 0.0,
            // El desfase evita que todos los soldados abran fuego a la vez.
            cooldown: 1.2 + posicion.x.rem_euclid(1.5),
        }
    }

    pub fn bullet(posicion: Vector2, direccion: Vector2) -> Self {
        Self {
            tipo: EntityType::Bullet,
            posicion,
            active: true,
            direccion,
            vida: 1.2,
            cooldown: 0.0,
        }
    }

    pub fn enemy_bullet(posicion: Vector2, direccion: Vector2) -> Self {
        Self {
            tipo: EntityType::EnemyBullet,
            posicion,
            active: true,
            direccion,
            vida: 3.4,
            cooldown: 0.0,
        }
    }
}
