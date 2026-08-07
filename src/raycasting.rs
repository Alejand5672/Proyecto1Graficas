use crate::config::{FOV, NUM_RAYOS, PASO_RAYO};
use crate::map::Map;
use raylib::prelude::*;

#[derive(Clone, Copy)]
pub struct Impacto {
    pub posicion: Vector2,
    pub distancia: f32,
    pub fila: i32,
    pub columna: i32,
}

pub fn lanzar_rayo(mapa: &Map, origen: Vector2, angulo: f32, distancia_maxima: f32) -> Impacto {
    let direccion = Vector2::new(angulo.cos(), angulo.sin());
    let mut distancia = 0.0;

    while distancia < distancia_maxima {
        distancia += PASO_RAYO;
        let posicion = origen + direccion * distancia;
        let columna = posicion.x.floor() as i32;
        let fila = posicion.y.floor() as i32;

        if Map::es_pared(mapa.celda_en(posicion.x, posicion.y)) {
            return Impacto {
                posicion,
                distancia,
                fila,
                columna,
            };
        }
    }

    Impacto {
        posicion: origen + direccion * distancia_maxima,
        distancia: distancia_maxima,
        fila: -1,
        columna: -1,
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
