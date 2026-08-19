mod config;
mod entity;
mod framebuffer;
mod map;
mod player;
mod raycasting;

use config::{
    DANO_ENEMIGO, DISTANCIA_ATAQUE_ENEMIGO, ENFRIAMIENTO_DISPARO, RADIO_ENEMIGO, RADIO_PICKUP,
    TAM_CELDA, VELOCIDAD_BALA, VELOCIDAD_BALA_ENEMIGA,
};
use entity::{Entity, EntityType};
use framebuffer::dibujar_frame;
use map::Map;
use player::Player;
use raycasting::{lanzar_rayo, lanzar_rayos};
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
    let mut textura_muro = rl
        .load_texture(&thread, "assets/wall_bunker.png")
        .expect("No se pudo cargar assets/wall_bunker.png");
    textura_muro.gen_texture_mipmaps();
    textura_muro.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);
    let mut jugador = Player::new(spawn);
    let mut vista_3d = true;
    let mut tiempo_disparo = 0.0_f32;
    rl.disable_cursor();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time().min(0.05);
        let enemigos_vivos = entidades
            .iter()
            .filter(|e| e.active && e.tipo == EntityType::Enemy)
            .count();
        if (jugador.vida <= 0 || enemigos_vivos == 0) && rl.is_key_pressed(KeyboardKey::KEY_R) {
            let (_, _, nuevas_entidades) = Map::cargar("laberinto1.txt");
            entidades = nuevas_entidades;
            jugador = Player::new(spawn);
            tiempo_disparo = 0.0;
        }
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

        let partida_activa = jugador.vida > 0 && enemigos_vivos > 0;
        if partida_activa {
            jugador.actualizar(&rl, &mapa, dt, vista_3d);
        }
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

        if partida_activa
            && jugador.has_weapon
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

        let mut disparos_enemigos = Vec::new();
        if partida_activa {
            for enemigo in entidades
                .iter_mut()
                .filter(|e| e.active && e.tipo == EntityType::Enemy)
            {
                enemigo.cooldown = (enemigo.cooldown - dt).max(0.0);
                let hacia_jugador = jugador.posicion - enemigo.posicion;
                let distancia = hacia_jugador.length();
                if distancia > 0.2
                    && distancia <= DISTANCIA_ATAQUE_ENEMIGO
                    && enemigo.cooldown <= 0.0
                {
                    let direccion = hacia_jugador / distancia;
                    let angulo = direccion.y.atan2(direccion.x);
                    let pared = lanzar_rayo(&mapa, enemigo.posicion, angulo, distancia);
                    if pared.distancia >= distancia - 0.06 {
                        disparos_enemigos.push(Entity::enemy_bullet(
                            enemigo.posicion + direccion * 0.36,
                            direccion,
                        ));
                        enemigo.cooldown = 1.0 + (enemigo.posicion.x * 0.17).rem_euclid(0.55);
                    }
                }
            }
        }
        entidades.extend(disparos_enemigos);

        for indice in 0..entidades.len() {
            let tipo = entidades[indice].tipo;
            if !entidades[indice].active
                || !matches!(tipo, EntityType::Bullet | EntityType::EnemyBullet)
            {
                continue;
            }
            let direccion = entidades[indice].direccion;
            let velocidad = if tipo == EntityType::Bullet {
                VELOCIDAD_BALA
            } else {
                VELOCIDAD_BALA_ENEMIGA
            };
            entidades[indice].posicion += direccion * velocidad * dt;
            entidades[indice].vida -= dt;
            let posicion_bala = entidades[indice].posicion;
            if entidades[indice].vida <= 0.0
                || Map::es_pared(mapa.celda_en(posicion_bala.x, posicion_bala.y))
            {
                entidades[indice].active = false;
                continue;
            }
            if tipo == EntityType::Bullet {
                let impacto = entidades
                    .iter()
                    .enumerate()
                    .find_map(|(otro_indice, entidad)| {
                        (otro_indice != indice
                            && entidad.active
                            && entidad.tipo == EntityType::Enemy
                            && entidad.posicion.distance_to(posicion_bala) < RADIO_ENEMIGO)
                            .then_some(otro_indice)
                    });
                if let Some(enemigo) = impacto {
                    entidades[indice].active = false;
                    entidades[enemigo].active = false;
                }
            } else if jugador.vida > 0
                && jugador.posicion.distance_to(posicion_bala) < RADIO_ENEMIGO
            {
                entidades[indice].active = false;
                jugador.vida = (jugador.vida - DANO_ENEMIGO).max(0);
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
