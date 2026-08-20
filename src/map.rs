use crate::config::RADIO_JUGADOR;
use crate::entity::Entity;
use raylib::prelude::*;
use std::fs;

pub struct Map {
    pub celdas: Vec<Vec<char>>,
    pub filas: usize,
    pub columnas: usize,
    pub salida: Option<(i32, i32)>,
}

impl Map {
    pub fn cargar(path: &str) -> (Self, Vector2, Vec<Entity>) {
        let texto = fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("No se pudo leer {path}: {error}"));
        let mut celdas: Vec<Vec<char>> =
            texto.lines().map(|linea| linea.chars().collect()).collect();
        let filas = celdas.len();
        let columnas = celdas.iter().map(Vec::len).max().unwrap_or(0);

        assert!(filas > 0 && columnas > 0, "{path} esta vacio");

        // Las líneas cortas conservan la abertura de salida del formato original.
        for fila in &mut celdas {
            fila.resize(columnas, ' ');
        }

        let mut spawn = Vector2::new(1.5, 1.5);
        let mut entidades = Vec::new();
        let mut salida = None;

        for (fila, linea) in celdas.iter_mut().enumerate() {
            for (columna, celda) in linea.iter_mut().enumerate() {
                match *celda {
                    'P' | '*' => {
                        spawn = Vector2::new(columna as f32 + 0.5, fila as f32 + 0.5);
                        *celda = ' ';
                    }
                    'G' => {
                        entidades.push(Entity::weapon(Vector2::new(
                            columna as f32 + 0.5,
                            fila as f32 + 0.5,
                        )));
                        *celda = ' ';
                    }
                    'M' => {
                        entidades.push(Entity::ammo(Vector2::new(
                            columna as f32 + 0.5,
                            fila as f32 + 0.5,
                        )));
                        *celda = ' ';
                    }
                    'H' => {
                        entidades.push(Entity::health(Vector2::new(
                            columna as f32 + 0.5,
                            fila as f32 + 0.5,
                        )));
                        *celda = ' ';
                    }
                    'E' => {
                        entidades.push(Entity::enemy(Vector2::new(
                            columna as f32 + 0.5,
                            fila as f32 + 0.5,
                        )));
                        *celda = ' ';
                    }
                    'B' => {
                        entidades.push(Entity::boss(Vector2::new(
                            columna as f32 + 0.5,
                            fila as f32 + 0.5,
                        )));
                        *celda = ' ';
                    }
                    'S' => salida = Some((columna as i32, fila as i32)),
                    _ => {}
                }
            }
        }

        if salida.is_none() {
            'buscar_salida: for fila in (0..filas).rev() {
                for columna in (0..columnas).rev() {
                    let borde =
                        fila == 0 || fila == filas - 1 || columna == 0 || columna == columnas - 1;
                    if borde && !Self::es_pared(celdas[fila][columna]) {
                        salida = Some((columna as i32, fila as i32));
                        break 'buscar_salida;
                    }
                }
            }
        }

        (
            Self {
                celdas,
                filas,
                columnas,
                salida,
            },
            spawn,
            entidades,
        )
    }

    pub fn es_pared(celda: char) -> bool {
        matches!(celda, '.' | '-' | '|')
    }

    pub fn celda_en(&self, x: f32, y: f32) -> char {
        if x < 0.0 || y < 0.0 {
            return '.';
        }

        self.celdas
            .get(y.floor() as usize)
            .and_then(|fila| fila.get(x.floor() as usize))
            .copied()
            .unwrap_or('.')
    }

    pub fn posicion_libre(&self, posicion: Vector2) -> bool {
        [
            (-RADIO_JUGADOR, -RADIO_JUGADOR),
            (RADIO_JUGADOR, -RADIO_JUGADOR),
            (-RADIO_JUGADOR, RADIO_JUGADOR),
            (RADIO_JUGADOR, RADIO_JUGADOR),
        ]
        .iter()
        .all(|(dx, dy)| !Self::es_pared(self.celda_en(posicion.x + dx, posicion.y + dy)))
    }
}
