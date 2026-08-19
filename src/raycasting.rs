use crate::config::{FOV, NUM_RAYOS};
use crate::map::Map;
use raylib::prelude::*;

#[derive(Clone, Copy)]
pub struct Impacto {
    pub posicion: Vector2,
    pub distancia: f32,
    pub fila: i32,
    pub columna: i32,
    pub lado_vertical: bool,
}

pub fn lanzar_rayo(mapa: &Map, origen: Vector2, angulo: f32, distancia_maxima: f32) -> Impacto {
    let direccion = Vector2::new(angulo.cos(), angulo.sin());
    let mut columna = origen.x.floor() as i32;
    let mut fila = origen.y.floor() as i32;
    let paso_x = if direccion.x < 0.0 { -1 } else { 1 };
    let paso_y = if direccion.y < 0.0 { -1 } else { 1 };
    let delta_x = if direccion.x.abs() < f32::EPSILON {
        f32::INFINITY
    } else {
        1.0 / direccion.x.abs()
    };
    let delta_y = if direccion.y.abs() < f32::EPSILON {
        f32::INFINITY
    } else {
        1.0 / direccion.y.abs()
    };
    let mut lado_x = if direccion.x < 0.0 {
        (origen.x - columna as f32) * delta_x
    } else {
        (columna as f32 + 1.0 - origen.x) * delta_x
    };
    let mut lado_y = if direccion.y < 0.0 {
        (origen.y - fila as f32) * delta_y
    } else {
        (fila as f32 + 1.0 - origen.y) * delta_y
    };

    while lado_x.min(lado_y) <= distancia_maxima {
        let lado_vertical = lado_x < lado_y;
        let distancia = if lado_vertical {
            lado_x += delta_x;
            columna += paso_x;
            lado_x - delta_x
        } else {
            lado_y += delta_y;
            fila += paso_y;
            lado_y - delta_y
        };

        if Map::es_pared(mapa.celda_en(columna as f32, fila as f32)) {
            return Impacto {
                posicion: origen + direccion * distancia,
                distancia,
                fila,
                columna,
                lado_vertical,
            };
        }
    }

    Impacto {
        posicion: origen + direccion * distancia_maxima,
        distancia: distancia_maxima,
        fila: -1,
        columna: -1,
        lado_vertical: false,
    }
}

pub fn lanzar_rayos(mapa: &Map, origen: Vector2, angulo: f32) -> Vec<(f32, Impacto)> {
    let distancia_maxima = mapa.columnas.max(mapa.filas) as f32 * 1.5;
    (0..NUM_RAYOS)
        .map(|i| {
            let proporcion = i as f32 / (NUM_RAYOS - 1) as f32;
            let angulo_rayo = angulo - FOV / 2.0 + proporcion * FOV;
            (
                angulo_rayo,
                lanzar_rayo(mapa, origen, angulo_rayo, distancia_maxima),
            )
        })
        .collect()
}
