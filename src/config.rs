use std::f32::consts::PI;

pub const TAM_CELDA: i32 = 36;
// Una columna por cada pocos píxeles evita que las paredes se vean como franjas.
pub const NUM_RAYOS: usize = 540;
pub const FOV: f32 = PI / 3.0;
pub const RADIO_JUGADOR: f32 = 0.20;
pub const RADIO_PICKUP: f32 = 0.55;
pub const RADIO_ENEMIGO: f32 = 0.32;
pub const VELOCIDAD_BALA: f32 = 9.0;
pub const ENFRIAMIENTO_DISPARO: f32 = 0.22;
