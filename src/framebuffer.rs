use crate::config::{FOV, NUM_RAYOS, TAM_CELDA};
use crate::entity::Entity;
use crate::map::Map;
use crate::player::Player;
use crate::raycasting::Impacto;
use raylib::prelude::*;

fn color_muro(fila: i32, columna: i32) -> Color {
    if (fila + columna) % 2 == 0 {
        Color::new(48, 125, 201, 255)
    } else {
        Color::new(46, 170, 92, 255)
    }
}

fn dibujar_linea_punteada(d: &mut RaylibDrawHandle, inicio: Vector2, fin: Vector2, color: Color) {
    let delta = fin - inicio;
    let longitud = delta.length();
    if longitud <= 0.0 {
        return;
    }

    let direccion = delta / longitud;
    let mut avance = 0.0;
    while avance < longitud {
        let final_punto = (avance + 7.0).min(longitud);
        d.draw_line_ex(
            inicio + direccion * avance,
            inicio + direccion * final_punto,
            2.0,
            color,
        );
        avance += 12.0;
    }
}

fn dibujar_pickup_arma(d: &mut RaylibDrawHandle, centro: Vector2, tamano: f32) {
    let ancho = tamano.max(8.0);
    let alto = (tamano * 0.55).max(5.0);
    d.draw_rectangle(
        (centro.x - ancho * 0.5) as i32,
        (centro.y - alto * 0.35) as i32,
        ancho as i32,
        (alto * 0.35) as i32,
        Color::new(62, 70, 82, 255),
    );
    d.draw_rectangle(
        (centro.x + ancho * 0.05) as i32,
        (centro.y - alto * 0.05) as i32,
        (ancho * 0.28) as i32,
        (alto * 0.65) as i32,
        Color::new(34, 38, 47, 255),
    );
    d.draw_circle_v(
        centro,
        (tamano * 0.08).max(2.0),
        Color::new(246, 200, 58, 255),
    );
}

fn dibujar_arma_primera_persona(d: &mut RaylibDrawHandle, ancho: f32, alto: f32) {
    let escala = (ancho.min(alto) / 500.0).max(0.75);
    let centro_x = ancho * 0.5;
    d.draw_rectangle(
        (centro_x - 72.0 * escala) as i32,
        (alto - 105.0 * escala) as i32,
        (144.0 * escala) as i32,
        (48.0 * escala) as i32,
        Color::new(45, 50, 60, 255),
    );
    d.draw_rectangle(
        (centro_x - 23.0 * escala) as i32,
        (alto - 63.0 * escala) as i32,
        (46.0 * escala) as i32,
        (63.0 * escala) as i32,
        Color::new(28, 31, 38, 255),
    );
    d.draw_rectangle(
        (centro_x - 56.0 * escala) as i32,
        (alto - 98.0 * escala) as i32,
        (112.0 * escala) as i32,
        (8.0 * escala) as i32,
        Color::new(102, 112, 126, 255),
    );
}

fn dibujar_jugador(d: &mut RaylibDrawHandle, jugador: &Player, textura_jugador: &Texture2D) {
    let centro = jugador.posicion * TAM_CELDA as f32;
    let tamano = 44.0;
    let origen = Vector2::new(tamano / 2.0, tamano / 2.0);
    d.draw_texture_pro(
        textura_jugador,
        Rectangle::new(
            0.0,
            0.0,
            textura_jugador.width() as f32,
            textura_jugador.height() as f32,
        ),
        Rectangle::new(centro.x, centro.y, tamano, tamano),
        origen,
        jugador.angulo.to_degrees(),
        Color::WHITE,
    );
}

fn dibujar_vista_3d(
    d: &mut RaylibDrawHandle,
    jugador: &Player,
    entidades: &[Entity],
    impactos: &[(f32, Impacto)],
) {
    let ancho = d.get_screen_width() as f32;
    let alto = d.get_screen_height() as f32;
    let mitad_alto = alto / 2.0;
    d.draw_rectangle(
        0,
        0,
        ancho as i32,
        mitad_alto as i32,
        Color::new(22, 31, 47, 255),
    );
    d.draw_rectangle(
        0,
        mitad_alto as i32,
        ancho as i32,
        mitad_alto as i32,
        Color::new(27, 33, 30, 255),
    );

    let ancho_estaca = ancho / NUM_RAYOS as f32;
    let distancia_plano = (ancho / 2.0) / (FOV / 2.0).tan();

    for (i, (angulo, impacto)) in impactos.iter().enumerate() {
        let distancia = (impacto.distancia * (*angulo - jugador.angulo).cos()).max(0.08);
        let alto_estaca = (distancia_plano / distancia).min(alto * 1.5);
        let techo = mitad_alto - alto_estaca / 2.0;
        let sombra = (1.0 / (1.0 + distancia * 0.10)).clamp(0.35, 1.0);
        let base = color_muro(impacto.fila, impacto.columna);
        let color = Color::new(
            (base.r as f32 * sombra) as u8,
            (base.g as f32 * sombra) as u8,
            (base.b as f32 * sombra) as u8,
            255,
        );

        d.draw_rectangle(
            (i as f32 * ancho_estaca) as i32,
            techo as i32,
            ancho_estaca.ceil() as i32 + 1,
            alto_estaca as i32,
            color,
        );
    }

    for entidad in entidades.iter().filter(|entidad| entidad.active) {
        let relativo = entidad.posicion - jugador.posicion;
        let profundidad = relativo.x * jugador.angulo.cos() + relativo.y * jugador.angulo.sin();
        let lateral = -relativo.x * jugador.angulo.sin() + relativo.y * jugador.angulo.cos();

        if profundidad > 0.05 {
            let pantalla_x = ancho * 0.5 + lateral / profundidad * distancia_plano;
            let indice_rayo = ((pantalla_x / ancho) * NUM_RAYOS as f32)
                .floor()
                .clamp(0.0, (NUM_RAYOS - 1) as f32) as usize;
            let (angulo_rayo, impacto_rayo) = impactos[indice_rayo];
            let distancia_muro = impacto_rayo.distancia * (angulo_rayo - jugador.angulo).cos();

            if pantalla_x >= 0.0 && pantalla_x < ancho && profundidad < distancia_muro {
                let tamano = (distancia_plano * 0.48 / profundidad).clamp(8.0, alto * 0.72);
                dibujar_pickup_arma(
                    d,
                    Vector2::new(pantalla_x, mitad_alto + tamano * 0.18),
                    tamano,
                );
            }
        }
    }

    if jugador.has_weapon {
        dibujar_arma_primera_persona(d, ancho, alto);
    }

    d.draw_text("Z / TAB: vista 2D", 14, 14, 20, Color::RAYWHITE);
}

fn dibujar_vista_2d(
    d: &mut RaylibDrawHandle,
    mapa: &Map,
    jugador: &Player,
    entidades: &[Entity],
    impactos: &[(f32, Impacto)],
    textura_jugador: &Texture2D,
) {
    let cuadricula = Color::new(35, 45, 60, 255);
    let amarillo = Color::new(246, 200, 58, 255);

    for fila in 0..mapa.filas as i32 {
        for columna in 0..mapa.columnas as i32 {
            let x = columna * TAM_CELDA;
            let y = fila * TAM_CELDA;
            if Map::es_pared(mapa.celdas[fila as usize][columna as usize]) {
                d.draw_rectangle(x, y, TAM_CELDA, TAM_CELDA, color_muro(fila, columna));
            } else {
                d.draw_rectangle_lines(x, y, TAM_CELDA, TAM_CELDA, cuadricula);
            }
        }
    }

    for entidad in entidades.iter().filter(|entidad| entidad.active) {
        dibujar_pickup_arma(
            d,
            entidad.posicion * TAM_CELDA as f32,
            TAM_CELDA as f32 * 0.55,
        );
    }

    let origen = jugador.posicion * TAM_CELDA as f32;
    for (_, impacto) in impactos {
        dibujar_linea_punteada(d, origen, impacto.posicion * TAM_CELDA as f32, amarillo);
    }
    dibujar_jugador(d, jugador, textura_jugador);

    if let Some((columna, fila)) = mapa.salida {
        let x = columna * TAM_CELDA;
        let y = fila * TAM_CELDA;
        d.draw_rectangle_lines_ex(
            Rectangle::new((x + 7) as f32, (y + 7) as f32, 22.0, 22.0),
            3.0,
            amarillo,
        );
        d.draw_text("E", x + 12, y + 8, 22, amarillo);
    }

    d.draw_text(
        "WASD: mover  Flechas/QE: girar  Z/TAB: vista 3D",
        12,
        10,
        18,
        Color::RAYWHITE,
    );
}

pub fn dibujar_frame(
    d: &mut RaylibDrawHandle,
    vista_3d: bool,
    mapa: &Map,
    jugador: &Player,
    entidades: &[Entity],
    impactos: &[(f32, Impacto)],
    textura_jugador: &Texture2D,
) {
    d.clear_background(Color::new(16, 22, 32, 255));
    if vista_3d {
        dibujar_vista_3d(d, jugador, entidades, impactos);
    } else {
        dibujar_vista_2d(d, mapa, jugador, entidades, impactos, textura_jugador);
    }
}
