mod config;
mod entity;
mod framebuffer;
mod map;
mod player;
mod raycasting;

use config::{RADIO_PICKUP, TAM_CELDA};
use entity::EntityType;
use framebuffer::dibujar_frame;
use map::Map;
use player::Player;
use raycasting::lanzar_rayos;
use raylib::prelude::*;

fn main() {
    let (mapa, spawn, mut entidades) = Map::cargar("laberinto.txt");
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
    let mut jugador = Player::new(spawn);
    let mut vista_3d = false;

    while !rl.window_should_close() {
        let dt = rl.get_frame_time().min(0.05);
        if rl.is_key_pressed(KeyboardKey::KEY_Z) || rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            vista_3d = !vista_3d;
        }

        jugador.actualizar(&rl, &mapa, dt);
        for entidad in &mut entidades {
            if entidad.active
                && entidad.tipo == EntityType::Weapon
                && jugador.posicion.distance_to(entidad.posicion) < RADIO_PICKUP
            {
                entidad.active = false;
                jugador.has_weapon = true;
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
        );
    }
}
