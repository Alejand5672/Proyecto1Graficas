use crate::config::{FOV, NUM_RAYOS, TAM_CELDA};
use crate::entity::{Entity, EntityType};
use crate::map::Map;
use crate::player::Player;
use crate::raycasting::Impacto;
use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum EstadoPantalla { Bienvenida, Jugando, NivelCompletado, Derrota, Victoria }

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

fn dibujar_municion(d: &mut RaylibDrawHandle, centro: Vector2, tamano: f32) {
    let ancho = (tamano * 0.42).max(7.0);
    let alto = (tamano * 0.26).max(5.0);
    d.draw_rectangle(
        (centro.x - ancho / 2.0) as i32,
        (centro.y - alto / 2.0) as i32,
        ancho as i32,
        alto as i32,
        Color::new(194, 139, 44, 255),
    );
    d.draw_rectangle_lines(
        (centro.x - ancho / 2.0) as i32,
        (centro.y - alto / 2.0) as i32,
        ancho as i32,
        alto as i32,
        Color::new(255, 220, 107, 255),
    );
}

fn dibujar_botiquin(d: &mut RaylibDrawHandle, centro: Vector2, tamano: f32) {
    let ancho = (tamano * 0.48).max(9.0);
    let alto = (tamano * 0.34).max(7.0);
    d.draw_rectangle(
        (centro.x - ancho / 2.0) as i32,
        (centro.y - alto / 2.0) as i32,
        ancho as i32,
        alto as i32,
        Color::new(224, 225, 212, 255),
    );
    d.draw_rectangle_lines(
        (centro.x - ancho / 2.0) as i32,
        (centro.y - alto / 2.0) as i32,
        ancho as i32,
        alto as i32,
        Color::new(121, 43, 39, 255),
    );
    let cruz = (alto * 0.24).max(2.0);
    d.draw_rectangle((centro.x - cruz * 0.5) as i32, (centro.y - alto * 0.34) as i32, cruz as i32, (alto * 0.68) as i32, Color::new(194, 47, 42, 255));
    d.draw_rectangle((centro.x - ancho * 0.19) as i32, (centro.y - cruz * 0.5) as i32, (ancho * 0.38) as i32, cruz as i32, Color::new(194, 47, 42, 255));
}

fn dibujar_bala(d: &mut RaylibDrawHandle, centro: Vector2, tamano: f32, enemiga: bool) {
    let color = if enemiga {
        Color::new(255, 72, 48, 255)
    } else {
        Color::new(255, 226, 97, 255)
    };
    let radio = (tamano * 0.11).clamp(3.0, 10.0);
    d.draw_circle_v(
        centro,
        radio * 2.2,
        Color::new(color.r, color.g, color.b, 45),
    );
    d.draw_circle_v(centro, radio, color);
    d.draw_circle_v(centro, (radio * 0.38).max(1.5), Color::WHITE);
    d.draw_line_ex(
        centro + Vector2::new(0.0, radio * 0.6),
        centro + Vector2::new(0.0, radio * 3.4),
        (radio * 0.65).max(2.0),
        Color::new(color.r, color.g, color.b, 150),
    );
}

fn dibujar_entidad(d: &mut RaylibDrawHandle, entidad: &Entity, centro: Vector2, tamano: f32, textura_enemigo: &Texture2D) {
    match entidad.tipo {
        EntityType::Weapon => dibujar_pickup_arma(d, centro, tamano),
        EntityType::Ammo => dibujar_municion(d, centro, tamano),
        EntityType::Health => dibujar_botiquin(d, centro, tamano),
        EntityType::Enemy | EntityType::Boss => d.draw_texture_pro(
            textura_enemigo,
            Rectangle::new(0.0, 0.0, textura_enemigo.width() as f32, textura_enemigo.height() as f32),
            Rectangle::new(centro.x, centro.y, tamano * if entidad.tipo == EntityType::Boss { 1.85 } else { 1.25 }, tamano * if entidad.tipo == EntityType::Boss { 1.85 } else { 1.25 }),
            Vector2::new(tamano * if entidad.tipo == EntityType::Boss { 0.925 } else { 0.625 }, tamano * if entidad.tipo == EntityType::Boss { 1.85 } else { 1.25 }),
            0.0,
            if entidad.tipo == EntityType::Boss { Color::new(255, 126, 112, 255) } else { Color::WHITE },
        ),
        EntityType::Bullet => dibujar_bala(d, centro, tamano, false),
        EntityType::EnemyBullet => dibujar_bala(d, centro, tamano, true),
    }
}

fn dibujar_arma_primera_persona(
    d: &mut RaylibDrawHandle,
    ancho: f32,
    alto: f32,
    textura_arma: &Texture2D,
    retroceso: f32,
    inclinacion: f32,
) {
    let altura_destino = (alto * 0.58).clamp(280.0, 430.0);
    let ancho_destino = altura_destino * textura_arma.width() as f32 / textura_arma.height() as f32;
    let fase_retroceso = (retroceso / 0.13).clamp(0.0, 1.0);
    let desplazamiento = (fase_retroceso * std::f32::consts::PI).sin() * 16.0;
    d.draw_texture_pro(
        textura_arma,
        Rectangle::new(
            0.0,
            0.0,
            textura_arma.width() as f32,
            textura_arma.height() as f32,
        ),
        Rectangle::new(
            ancho * 0.5,
            // Arma y retícula comparten el desplazamiento de la cámara.
            alto + 8.0 + desplazamiento - inclinacion,
            ancho_destino,
            altura_destino,
        ),
        Vector2::new(ancho_destino * 0.5, altura_destino),
        0.0,
        Color::WHITE,
    );

    if retroceso > 0.075 {
        let boca = Vector2::new(
            ancho * 0.5,
            alto + 8.0 + desplazamiento - inclinacion - altura_destino * 0.88,
        );
        d.draw_circle_v(boca, 18.0, Color::new(255, 166, 38, 105));
        d.draw_circle_v(boca, 8.0, Color::new(255, 241, 158, 245));
        for i in 0..8 {
            let angulo = i as f32 * std::f32::consts::PI / 4.0;
            let direccion = Vector2::new(angulo.cos(), angulo.sin());
            d.draw_line_ex(
                boca + direccion * 7.0,
                boca + direccion * 27.0,
                3.0,
                Color::new(255, 196, 61, 210),
            );
        }
    }
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

fn dibujar_minimapa(d: &mut RaylibDrawHandle, mapa: &Map, jugador: &Player, entidades: &[Entity]) {
    let escala = 4.0_f32;
    let ancho = mapa.columnas as f32 * escala;
    let alto = mapa.filas as f32 * escala;
    let margen = 12.0;
    let origen = Vector2::new(d.get_screen_width() as f32 - ancho - margen, margen);

    d.draw_rectangle(
        (origen.x - 3.0) as i32,
        (origen.y - 3.0) as i32,
        (ancho + 6.0) as i32,
        (alto + 6.0) as i32,
        Color::new(8, 12, 18, 210),
    );

    for fila in 0..mapa.filas {
        for columna in 0..mapa.columnas {
            let color = if Map::es_pared(mapa.celdas[fila][columna]) {
                Color::new(91, 122, 153, 255)
            } else {
                Color::new(34, 43, 50, 255)
            };
            d.draw_rectangle(
                (origen.x + columna as f32 * escala) as i32,
                (origen.y + fila as f32 * escala) as i32,
                escala.ceil() as i32,
                escala.ceil() as i32,
                color,
            );
        }
    }

    for entidad in entidades.iter().filter(|entidad| entidad.active) {
        let color = match entidad.tipo {
            EntityType::Enemy => Color::new(224, 82, 68, 255),
            EntityType::Boss => Color::new(255, 46, 38, 255),
            EntityType::Weapon => Color::new(246, 200, 58, 255),
            EntityType::Ammo => Color::new(239, 171, 57, 255),
            EntityType::Health => Color::new(240, 76, 67, 255),
            EntityType::Bullet => Color::new(255, 240, 180, 255),
            EntityType::EnemyBullet => Color::new(255, 64, 48, 255),
        };
        d.draw_circle_v(origen + entidad.posicion * escala, 2.0, color);
    }

    let posicion = origen + jugador.posicion * escala;
    let frente = Vector2::new(jugador.angulo.cos(), jugador.angulo.sin());
    d.draw_line_ex(posicion, posicion + frente * 10.0, 2.0, Color::RAYWHITE);
    d.draw_circle_v(posicion, 3.0, Color::new(80, 219, 255, 255));
    d.draw_rectangle_lines(
        (origen.x - 3.0) as i32,
        (origen.y - 3.0) as i32,
        (ancho + 6.0) as i32,
        (alto + 6.0) as i32,
        Color::RAYWHITE,
    );
}

fn dibujar_vista_3d(
    d: &mut RaylibDrawHandle,
    mapa: &Map,
    jugador: &Player,
    entidades: &[Entity],
    impactos: &[(f32, Impacto)],
    textura_muro: &Texture2D,
    textura_arma: &Texture2D,
    textura_enemigo: &Texture2D,
) {
    let ancho = d.get_screen_width() as f32;
    let alto = d.get_screen_height() as f32;
    // Al mirar abajo el horizonte sube; al mirar arriba baja.
    let horizonte = (alto / 2.0 - jugador.inclinacion).clamp(alto * 0.18, alto * 0.82);
    let bandas = 18;
    for banda in 0..bandas {
        let t = banda as f32 / bandas as f32;
        let y = (t * horizonte) as i32;
        let h = (horizonte / bandas as f32).ceil() as i32 + 1;
        d.draw_rectangle(
            0,
            y,
            ancho as i32,
            h,
            Color::new(
                (13.0 + 15.0 * t) as u8,
                (21.0 + 20.0 * t) as u8,
                (34.0 + 25.0 * t) as u8,
                255,
            ),
        );
        d.draw_rectangle(
            0,
            (horizonte + t * (alto - horizonte)) as i32,
            ancho as i32,
            ((alto - horizonte) / bandas as f32).ceil() as i32 + 1,
            Color::new(
                (37.0 - 19.0 * t) as u8,
                (42.0 - 20.0 * t) as u8,
                (40.0 - 18.0 * t) as u8,
                255,
            ),
        );
    }

    let ancho_estaca = ancho / NUM_RAYOS as f32;
    let distancia_plano = (ancho / 2.0) / (FOV / 2.0).tan();

    for (i, (angulo, impacto)) in impactos.iter().enumerate() {
        let distancia = (impacto.distancia * (*angulo - jugador.angulo).cos()).max(0.08);
        // Proyección perspectiva real. Cuando el muro supera la pantalla se recorta
        // también la región de textura, en vez de limitar artificialmente su escala.
        let alto_estaca = distancia_plano / distancia;
        let techo = horizonte - alto_estaca / 2.0;
        let inicio_visible = techo.max(0.0);
        let fin_visible = (techo + alto_estaca).min(alto);
        let alto_visible = (fin_visible - inicio_visible).max(0.0);
        if alto_visible <= 0.0 {
            continue;
        }
        let sombra = (1.0 / (1.0 + distancia * 0.10)).clamp(0.35, 1.0);
        let coordenada_textura = if impacto.lado_vertical {
            impacto.posicion.y.rem_euclid(1.0)
        } else {
            impacto.posicion.x.rem_euclid(1.0)
        };
        let textura_x = (coordenada_textura * textura_muro.width() as f32)
            .clamp(0.0, textura_muro.width() as f32 - 1.0);
        let textura_y = ((inicio_visible - techo) / alto_estaca) * textura_muro.height() as f32;
        let alto_textura = (alto_visible / alto_estaca) * textura_muro.height() as f32;
        let variacion = match (impacto.fila + impacto.columna).rem_euclid(3) {
            0 => (0.92, 0.98, 1.0),
            1 => (0.82, 0.94, 0.96),
            _ => (0.96, 0.90, 0.82),
        };
        let brillo = 255.0 * sombra;
        d.draw_texture_pro(
            textura_muro,
            Rectangle::new(textura_x, textura_y, 1.0, alto_textura),
            Rectangle::new(
                i as f32 * ancho_estaca,
                inicio_visible,
                ancho_estaca.ceil() + 1.0,
                alto_visible,
            ),
            Vector2::zero(),
            0.0,
            Color::new(
                (brillo * variacion.0) as u8,
                (brillo * variacion.1) as u8,
                (brillo * variacion.2) as u8,
                255,
            ),
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
                dibujar_entidad(
                    d,
                    entidad,
                    // El origen del sprite está en sus botas: queda sobre el suelo.
                    Vector2::new(pantalla_x, horizonte + tamano * 0.5),
                    tamano,
                    textura_enemigo,
                );
            }
        }
    }

    if jugador.has_weapon {
        dibujar_arma_primera_persona(
            d,
            ancho,
            alto,
            textura_arma,
            jugador.retroceso,
            jugador.inclinacion,
        );
    }

    dibujar_minimapa(d, mapa, jugador, entidades);

    // La pequeña sacudida mantiene la retícula ligada al retroceso del arma.
    let fase_retroceso = (jugador.retroceso / 0.13).clamp(0.0, 1.0);
    let sacudida_mira = (fase_retroceso * std::f32::consts::PI).sin() * 4.0;
    let centro = Vector2::new(ancho * 0.5, horizonte + sacudida_mira);
    let mira = Color::new(235, 244, 238, 210);
    d.draw_line_ex(
        centro - Vector2::new(11.0, 0.0),
        centro - Vector2::new(3.0, 0.0),
        2.0,
        mira,
    );
    d.draw_line_ex(
        centro + Vector2::new(3.0, 0.0),
        centro + Vector2::new(11.0, 0.0),
        2.0,
        mira,
    );
    d.draw_line_ex(
        centro - Vector2::new(0.0, 11.0),
        centro - Vector2::new(0.0, 3.0),
        2.0,
        mira,
    );
    d.draw_line_ex(
        centro + Vector2::new(0.0, 3.0),
        centro + Vector2::new(0.0, 11.0),
        2.0,
        mira,
    );

}

fn dibujar_vista_2d(
    d: &mut RaylibDrawHandle,
    mapa: &Map,
    jugador: &Player,
    entidades: &[Entity],
    impactos: &[(f32, Impacto)],
    textura_jugador: &Texture2D,
    textura_enemigo: &Texture2D,
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
        dibujar_entidad(
            d,
            entidad,
            entidad.posicion * TAM_CELDA as f32,
            TAM_CELDA as f32 * 0.55,
            textura_enemigo,
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
        &format!(
            "WASD: mover  Flechas/QE: girar  ESPACIO: disparar  Municion: {}",
            jugador.municion
        ),
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
    textura_enemigo: &Texture2D,
    textura_muro: &Texture2D,
    textura_arma: &Texture2D,
    estado: EstadoPantalla,
    nivel: u8,
) {
    d.clear_background(Color::new(16, 22, 32, 255));
    if vista_3d {
        dibujar_vista_3d(
            d,
            mapa,
            jugador,
            entidades,
            impactos,
            textura_muro,
            textura_arma,
            textura_enemigo,
        );
    } else {
        dibujar_vista_2d(d, mapa, jugador, entidades, impactos, textura_jugador, textura_enemigo);
    }
    let enemigos = entidades
        .iter()
        .filter(|e| e.active && matches!(e.tipo, EntityType::Enemy | EntityType::Boss))
        .count();
    let vida = jugador.vida.clamp(0, 100);
    let color_vida = if vida > 60 {
        Color::new(61, 211, 109, 255)
    } else if vida > 30 {
        Color::new(244, 184, 65, 255)
    } else {
        Color::new(239, 70, 62, 255)
    };
    d.draw_rectangle(10, 10, 286, 88, Color::new(5, 9, 15, 220));
    d.draw_rectangle_lines(10, 10, 286, 88, Color::new(128, 153, 165, 255));
    d.draw_text("VIDA", 20, 18, 21, Color::RAYWHITE);
    d.draw_rectangle(82, 20, 150, 18, Color::new(49, 28, 28, 255));
    d.draw_rectangle(82, 20, (150.0 * vida as f32 / 100.0) as i32, 18, color_vida);
    d.draw_rectangle_lines(82, 20, 150, 18, Color::RAYWHITE);
    d.draw_text(&format!("{vida}/100"), 240, 18, 20, color_vida);
    d.draw_text(
        &format!("MUNICION  {}", jugador.municion),
        20,
        48,
        19,
        Color::new(255, 215, 91, 255),
    );
    d.draw_text(
        &format!("ENEMIGOS  {enemigos}"),
        155,
        48,
        19,
        Color::new(255, 151, 91, 255),
    );
    if let Some(jefe) = entidades.iter().find(|e| e.active && e.tipo == EntityType::Boss) {
        let vida_jefe = jefe.vida.clamp(0.0, 18.0);
        d.draw_text("JEFE", 310, 18, 20, Color::new(255, 126, 112, 255));
        d.draw_rectangle(310, 42, 165, 15, Color::new(55, 25, 25, 255));
        d.draw_rectangle(310, 42, (165.0 * vida_jefe / 18.0) as i32, 15, Color::new(222, 57, 49, 255));
        d.draw_rectangle_lines(310, 42, 165, 15, Color::RAYWHITE);
    }
    d.draw_text(
        "Mouse/stick: mirar   Clic/RT: disparar",
        20,
        73,
        13,
        Color::new(184, 203, 211, 255),
    );
    let alto_pantalla = d.get_screen_height();
    d.draw_rectangle(8, alto_pantalla - 32, 90, 24, Color::new(7, 10, 15, 190));
    d.draw_fps(15, alto_pantalla - 29);
    if let Some((titulo, detalle, color)) = match estado {
        EstadoPantalla::Bienvenida => Some(("WICHO SLUG", "1/2 o flechas: elegir nivel    ENTER: jugar", Color::new(255, 215, 91, 255))),
        EstadoPantalla::NivelCompletado => Some(("NIVEL 1 SUPERADO", "ENTER: siguiente nivel    M: menu", Color::new(95, 235, 137, 255))),
        EstadoPantalla::Derrota => Some(("MISION FALLIDA", "Presiona R para reintentar", Color::new(245, 76, 62, 255))),
        EstadoPantalla::Victoria => Some(("JEFE DERROTADO", "VICTORIA - Presiona R para volver al menu", Color::new(95, 235, 137, 255))),
        EstadoPantalla::Jugando => None,
    } {
        let ancho = d.get_screen_width(); let alto = d.get_screen_height();
        d.draw_rectangle(0, 0, ancho, alto, Color::new(5, 7, 10, 195));
        let t = d.measure_text(titulo, 42); d.draw_text(titulo, (ancho - t) / 2, alto / 2 - 50, 42, color);
        let s = d.measure_text(detalle, 21); d.draw_text(detalle, (ancho - s) / 2, alto / 2 + 15, 21, Color::RAYWHITE);
        if estado == EstadoPantalla::Bienvenida { let n = format!("NIVEL SELECCIONADO: {nivel}"); let w = d.measure_text(&n, 18); d.draw_text(&n, (ancho - w) / 2, alto / 2 + 55, 18, Color::new(184, 203, 211, 255)); }
    }
}
