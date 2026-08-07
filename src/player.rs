use crate::map::Map;
use raylib::prelude::*;

pub struct Player {
    pub posicion: Vector2,
    pub angulo: f32,
    pub velocidad: f32,
    pub has_weapon: bool,
}

impl Player {
    pub fn new(posicion: Vector2) -> Self {
        Self {
            posicion,
            angulo: 0.0,
            velocidad: 2.4,
            has_weapon: false,
        }
    }

    pub fn actualizar(&mut self, rl: &RaylibHandle, mapa: &Map, dt: f32) {
        let velocidad_giro = 2.2 * dt;
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_Q) {
            self.angulo -= velocidad_giro;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_E) {
            self.angulo += velocidad_giro;
        }

        let frente = Vector2::new(self.angulo.cos(), self.angulo.sin());
        let derecha = Vector2::new(-frente.y, frente.x);
        let mut movimiento = Vector2::zero();

        if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP) {
            movimiento += frente;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN) {
            movimiento -= frente;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            movimiento -= derecha;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            movimiento += derecha;
        }

        if movimiento.length() > 0.0 {
            movimiento = movimiento.normalized() * (self.velocidad * dt);
            let intento_x = Vector2::new(self.posicion.x + movimiento.x, self.posicion.y);
            if mapa.posicion_libre(intento_x) {
                self.posicion.x = intento_x.x;
            }

            let intento_y = Vector2::new(self.posicion.x, self.posicion.y + movimiento.y);
            if mapa.posicion_libre(intento_y) {
                self.posicion.y = intento_y.y;
            }
        }
    }
}
