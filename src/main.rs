mod config;
mod entity;
mod framebuffer;
mod map;
mod player;
mod raycasting;

use config::{ENFRIAMIENTO_DISPARO, RADIO_ENEMIGO, RADIO_PICKUP, TAM_CELDA, VELOCIDAD_BALA};
use entity::{Entity, EntityType};
use framebuffer::dibujar_frame;
use map::Map;
use player::Player;
use raycasting::lanzar_rayos;
use raylib::prelude::*;

fn main() {
    let (mapa, spawn, mut entidades) = Map::cargar("laberinto1.txt");
    let ancho_ventana = mapa.columnas as i32 * TAM_CELDA;
    let alto_ventana = mapa.filas as i32 * TAM_CELDA;

    let (mut rl, thread) = raylib::init()
        .size(ancho_ventana, alto_ventana)
        .title("Maze Runner - raycasting")
        .build();
    rl.set_target_fps(60);

    let textura_jugador = rl
        .load_texture(&thread, "assets/player.png")
        .expect("No se pudo cargar assets/player.png");
    let textura_muro = rl
        .load_texture(&thread, "assets/wall_bunker.png")
        .expect("No se pudo cargar assets/wall_bunker.png");
    let mut jugador = Player::new(spawn);
    let mut vista_3d = true;
    let mut tiempo_disparo = 0.0_f32;
    rl.disable_cursor();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time().min(0.05);
        if rl.is_key_pressed(KeyboardKey::KEY_Z)
            || rl.is_key_pressed(KeyboardKey::KEY_TAB)
            || (rl.is_gamepad_available(0)
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT))
        {
            vista_3d = !vista_3d;
            if vista_3d {
                rl.disable_cursor();
            } else {
                rl.enable_cursor();
            }
        }

        jugador.actualizar(&rl, &mapa, dt, vista_3d);
        tiempo_disparo = (tiempo_disparo - dt).max(0.0);
        for entidad in &mut entidades {
            if entidad.active && jugador.posicion.distance_to(entidad.posicion) < RADIO_PICKUP {
                match entidad.tipo {
                    EntityType::Weapon => {
                        entidad.active = false;
                        jugador.has_weapon = true;
                        jugador.municion += 12;
                    }
                    EntityType::Ammo => {
                        entidad.active = false;
                        jugador.municion += 8;
                    }
                    _ => {}
                }
            }
        }

        if jugador.has_weapon
            && jugador.municion > 0
            && tiempo_disparo <= 0.0
            && (rl.is_key_pressed(KeyboardKey::KEY_SPACE)
                || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                || (rl.is_gamepad_available(0)
                    && rl.is_gamepad_button_pressed(
                        0,
                        GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2,
                    )))
        {
            let direccion = Vector2::new(jugador.angulo.cos(), jugador.angulo.sin());
            entidades.push(Entity::bullet(
                jugador.posicion + direccion * 0.28,
                direccion,
            ));
            jugador.municion -= 1;
            tiempo_disparo = ENFRIAMIENTO_DISPARO;
        }

        for indice in 0..entidades.len() {
            if !entidades[indice].active || entidades[indice].tipo != EntityType::Bullet {
                continue;
            }
            let direccion = entidades[indice].direccion;
            entidades[indice].posicion += direccion * VELOCIDAD_BALA * dt;
            entidades[indice].vida -= dt;
            if entidades[indice].vida <= 0.0 || !mapa.posicion_libre(entidades[indice].posicion) {
                entidades[indice].active = false;
                continue;
            }
            let impacto = entidades
                .iter()
                .enumerate()
                .find_map(|(otro_indice, entidad)| {
                    (otro_indice != indice
                        && entidad.active
                        && entidad.tipo == EntityType::Enemy
                        && entidad.posicion.distance_to(entidades[indice].posicion) < RADIO_ENEMIGO)
                        .then_some(otro_indice)
                });
            if let Some(enemigo) = impacto {
                entidades[indice].active = false;
                entidades[enemigo].active = false;
            }
        }

        let impactos = lanzar_rayos(&mapa, jugador.posicion, jugador.angulo);
        let mut d = rl.begin_drawing(&thread);
        dibujar_frame(
            &mut d,
            vista_3d,
            &mapa,
            &jugador,
            &entidades,
            &impactos,
            &textura_jugador,
            &textura_muro,
        );
    }
}
