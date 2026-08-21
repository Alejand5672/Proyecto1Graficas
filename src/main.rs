mod config; mod entity; mod framebuffer; mod map; mod player; mod raycasting;
use config::{DANO_ENEMIGO, DISTANCIA_ATAQUE_ENEMIGO, ENFRIAMIENTO_DISPARO, RADIO_ENEMIGO, RADIO_PICKUP, TAM_CELDA, VELOCIDAD_BALA, VELOCIDAD_BALA_ENEMIGA};
use entity::{Entity, EntityType}; use framebuffer::{dibujar_frame, EstadoPantalla}; use map::Map; use player::Player; use raycasting::{lanzar_rayo, lanzar_rayos}; use raylib::prelude::*;

fn cargar_nivel(nivel: u8) -> (Map, Vector2, Vec<Entity>) { Map::cargar(if nivel == 1 { "laberinto1.txt" } else { "laberinto2.txt" }) }
fn es_combatiente(tipo: EntityType) -> bool { matches!(tipo, EntityType::Enemy | EntityType::Boss) }

fn crear_onda(frecuencia: f32, duracion: f32, descendente: bool) -> Vec<u8> {
    let tasa = 22_050_u32;
    let muestras = (tasa as f32 * duracion) as u32;
    let mut wav = Vec::with_capacity(44 + muestras as usize * 2);
    let bytes_datos = muestras * 2;
    wav.extend_from_slice(b"RIFF"); wav.extend_from_slice(&(36 + bytes_datos).to_le_bytes()); wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes()); wav.extend_from_slice(&1_u16.to_le_bytes()); wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&tasa.to_le_bytes()); wav.extend_from_slice(&(tasa * 2).to_le_bytes()); wav.extend_from_slice(&2_u16.to_le_bytes()); wav.extend_from_slice(&16_u16.to_le_bytes()); wav.extend_from_slice(b"data"); wav.extend_from_slice(&bytes_datos.to_le_bytes());
    for i in 0..muestras { let t = i as f32 / tasa as f32; let progreso = i as f32 / muestras as f32; let tono = if descendente { frecuencia * (1.0 - progreso * 0.55) } else { frecuencia }; let envolvente = (1.0 - progreso).powi(2); let muestra = ((t * tono * std::f32::consts::TAU).sin() * envolvente * 0.45 * i16::MAX as f32) as i16; wav.extend_from_slice(&muestra.to_le_bytes()); }
    wav
}

fn crear_disparo() -> Vec<u8> {
    let tasa = 44_100_u32;
    let muestras = (tasa as f32 * 0.18) as u32;
    let bytes_datos = muestras * 2;
    let mut wav = Vec::with_capacity(44 + bytes_datos as usize);
    wav.extend_from_slice(b"RIFF"); wav.extend_from_slice(&(36 + bytes_datos).to_le_bytes()); wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes()); wav.extend_from_slice(&1_u16.to_le_bytes()); wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&tasa.to_le_bytes()); wav.extend_from_slice(&(tasa * 2).to_le_bytes()); wav.extend_from_slice(&2_u16.to_le_bytes()); wav.extend_from_slice(&16_u16.to_le_bytes()); wav.extend_from_slice(b"data"); wav.extend_from_slice(&bytes_datos.to_le_bytes());
    let mut semilla = 0xA3C5_9F17_u32;
    for i in 0..muestras {
        let t = i as f32 / tasa as f32;
        semilla = semilla.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let ruido = ((semilla >> 9) as f32 / 8_388_608.0) - 1.0;
        let fogonazo = (-t * 32.0).exp();
        let chasquido = ruido * (-t * 24.0).exp() * 0.70;
        let golpe = (t * (105.0 - t * 300.0).max(35.0) * std::f32::consts::TAU).sin() * fogonazo * 0.52;
        let cola = ruido * (-(t - 0.035).max(0.0) * 18.0).exp() * 0.12;
        let muestra = ((chasquido + golpe + cola) * i16::MAX as f32 * 0.72).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
        wav.extend_from_slice(&muestra.to_le_bytes());
    }
    wav
}

fn main() {
    let (mut mapa, mut spawn, mut entidades) = cargar_nivel(1);
    let (mut rl, thread) = raylib::init().size(mapa.columnas as i32 * TAM_CELDA, mapa.filas as i32 * TAM_CELDA).title("Wicho Slug").build();
    rl.maximize_window();
    rl.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));
    rl.set_target_fps(60);
    let textura_jugador = rl.load_texture(&thread, "assets/player.png").expect("No se pudo cargar assets/player.png");
    let textura_enemigo = rl.load_texture(&thread, "assets/enemy_soldier.png").expect("No se pudo cargar assets/enemy_soldier.png");
    let mut textura_muro_nivel1 = rl.load_texture(&thread, "assets/wall_bunker.png").expect("No se pudo cargar assets/wall_bunker.png"); textura_muro_nivel1.gen_texture_mipmaps(); textura_muro_nivel1.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);
    let mut textura_muro_nivel2 = rl.load_texture(&thread, "assets/wall_war_bunker.png").expect("No se pudo cargar assets/wall_war_bunker.png"); textura_muro_nivel2.gen_texture_mipmaps(); textura_muro_nivel2.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);
    let textura_arma = rl.load_texture(&thread, "assets/first_person_hands.png").expect("No se pudo cargar assets/first_person_hands.png");
    let textura_fuego = rl.load_texture(&thread, "assets/fire.png").expect("No se pudo cargar assets/fire.png");
    let audio = RaylibAudio::init_audio_device().expect("No se pudo inicializar el audio");
    let sonido_disparo = { let onda = audio.new_wave_from_memory(".wav", &crear_disparo()).expect("No se pudo crear el sonido de disparo"); audio.new_sound_from_wave(&onda).expect("No se pudo cargar el sonido de disparo") };
    let sonido_dano = { let onda = audio.new_wave_from_memory(".wav", &crear_onda(96.0, 0.22, true)).expect("No se pudo crear el sonido de dano"); audio.new_sound_from_wave(&onda).expect("No se pudo cargar el sonido de dano") };
    // Copia autorizada de la canción del Nivel 1.
    let musica_nivel1 = audio.new_music("assets/music/Taylor Swift - Style.mp3").ok();
    if let Some(musica) = &musica_nivel1 { musica.set_volume(0.32); }
    let musica_nivel2 = audio.new_music("assets/music/Taylor Swift - Blank Space.mp3").ok();
    if let Some(musica) = &musica_nivel2 { musica.set_volume(0.30); }
    let musica_menu = audio.new_music("assets/music/Taylor Swift - Look What You Made Me Do.mp3").ok();
    if let Some(musica) = &musica_menu { musica.set_volume(0.24); }
    let mut jugador = Player::new(spawn); let mut nivel = 1_u8; let mut estado = EstadoPantalla::Bienvenida; let mut opcion_menu = 0_i32; let mut opcion_nivel = 0_i32; let mut opcion_pausa = 0_i32; let mut tiempo_disparo = 0.0_f32; let mut salir = false; rl.enable_cursor();
    while !rl.window_should_close() && !salir {
        let dt = rl.get_frame_time().min(0.05);
        if let Some(musica) = &musica_nivel1 {
            if estado == EstadoPantalla::Jugando && nivel == 1 {
                if !musica.is_stream_playing() { musica.play_stream(); }
                musica.update_stream();
            } else if musica.is_stream_playing() {
                musica.stop_stream();
            }
        }
        if let Some(musica) = &musica_nivel2 {
            if estado == EstadoPantalla::Jugando && nivel == 2 {
                if !musica.is_stream_playing() { musica.play_stream(); }
                musica.update_stream();
            } else if musica.is_stream_playing() {
                musica.stop_stream();
            }
        }
        if let Some(musica) = &musica_menu {
            if matches!(estado, EstadoPantalla::Bienvenida | EstadoPantalla::SeleccionNivel | EstadoPantalla::Instrucciones) {
                if !musica.is_stream_playing() { musica.play_stream(); }
                musica.update_stream();
            } else if musica.is_stream_playing() {
                musica.stop_stream();
            }
        }
        match estado {
            EstadoPantalla::Bienvenida => {
                if rl.is_key_pressed(KeyboardKey::KEY_UP) { opcion_menu = (opcion_menu + 2) % 3; }
                if rl.is_key_pressed(KeyboardKey::KEY_DOWN) { opcion_menu = (opcion_menu + 1) % 3; }
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    match opcion_menu {
                        0 => { opcion_nivel = 0; estado = EstadoPantalla::SeleccionNivel; }
                        1 => estado = EstadoPantalla::Instrucciones,
                        _ => salir = true,
                    }
                }
            }
            EstadoPantalla::SeleccionNivel => {
                if rl.is_key_pressed(KeyboardKey::KEY_UP) { opcion_nivel = (opcion_nivel + 2) % 3; }
                if rl.is_key_pressed(KeyboardKey::KEY_DOWN) { opcion_nivel = (opcion_nivel + 1) % 3; }
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    if opcion_nivel == 2 { estado = EstadoPantalla::Bienvenida; }
                    else { nivel = if opcion_nivel == 0 { 1 } else { 2 }; (mapa, spawn, entidades) = cargar_nivel(nivel); jugador = Player::new(spawn); tiempo_disparo = 0.0; estado = EstadoPantalla::Jugando; rl.disable_cursor(); }
                }
            }
            EstadoPantalla::Pausa => {
                if rl.is_key_pressed(KeyboardKey::KEY_UP) || rl.is_key_pressed(KeyboardKey::KEY_DOWN) { opcion_pausa = (opcion_pausa + 1) % 2; }
                if rl.is_key_pressed(KeyboardKey::KEY_P) || (rl.is_key_pressed(KeyboardKey::KEY_ENTER) && opcion_pausa == 0) { estado = EstadoPantalla::Jugando; rl.disable_cursor(); }
                else if rl.is_key_pressed(KeyboardKey::KEY_ENTER) && opcion_pausa == 1 { salir = true; }
            }
            EstadoPantalla::Instrucciones => if rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_key_pressed(KeyboardKey::KEY_M) { estado = EstadoPantalla::Bienvenida; },
            EstadoPantalla::NivelCompletado => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    nivel = 2; (mapa, spawn, entidades) = cargar_nivel(nivel); jugador = Player::new(spawn); tiempo_disparo = 0.0; estado = EstadoPantalla::Jugando; rl.disable_cursor();
                } else if rl.is_key_pressed(KeyboardKey::KEY_M) {
                    estado = EstadoPantalla::Bienvenida; rl.enable_cursor();
                }
            }
            EstadoPantalla::Derrota => if rl.is_key_pressed(KeyboardKey::KEY_R) { (mapa, spawn, entidades) = cargar_nivel(nivel); jugador = Player::new(spawn); tiempo_disparo = 0.0; estado = EstadoPantalla::Jugando; rl.disable_cursor(); },
            EstadoPantalla::Victoria => if rl.is_key_pressed(KeyboardKey::KEY_R) { estado = EstadoPantalla::Bienvenida; rl.enable_cursor(); },
            EstadoPantalla::Jugando => {
                if rl.is_key_pressed(KeyboardKey::KEY_P) || rl.is_key_down(KeyboardKey::KEY_P) || rl.is_key_pressed(KeyboardKey::KEY_F1) { opcion_pausa = 0; estado = EstadoPantalla::Pausa; rl.enable_cursor(); continue; }
                jugador.actualizar(&rl, &mapa, dt, true); tiempo_disparo = (tiempo_disparo - dt).max(0.0); jugador.retroceso = (jugador.retroceso - dt).max(0.0);
                for e in &mut entidades { if e.active && jugador.posicion.distance_to(e.posicion) < RADIO_PICKUP { match e.tipo { EntityType::Weapon => { e.active=false; jugador.has_weapon=true; jugador.municion+=30; }, EntityType::Ammo => { e.active=false; jugador.municion+=15; }, EntityType::Health => { e.active=false; jugador.vida=(jugador.vida+35).min(100); }, _=>{} } } }
                if jugador.has_weapon && jugador.municion > 0 && tiempo_disparo <= 0.0 && (rl.is_key_pressed(KeyboardKey::KEY_SPACE) || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)) { let dir=Vector2::new(jugador.angulo.cos(),jugador.angulo.sin()); entidades.push(Entity::bullet(jugador.posicion+dir*0.28,dir)); jugador.municion-=1; tiempo_disparo=ENFRIAMIENTO_DISPARO; jugador.retroceso=0.13; sonido_disparo.play(); }
                let mut disparos=Vec::new();
                for e in entidades.iter_mut().filter(|e| e.active && (es_combatiente(e.tipo) || e.tipo == EntityType::Fire)) { e.animacion += dt * if e.tipo == EntityType::Boss { 4.6 } else { 6.0 }; if !es_combatiente(e.tipo) { continue; } e.cooldown=(e.cooldown-dt).max(0.0); let hacia=jugador.posicion-e.posicion; let distancia=hacia.length(); if distancia>0.2 && distancia<=DISTANCIA_ATAQUE_ENEMIGO && e.cooldown<=0.0 { let dir=hacia/distancia; let angulo=dir.y.atan2(dir.x); if lanzar_rayo(&mapa,e.posicion,angulo,distancia).distancia>=distancia-0.06 { let dispersion=(e.posicion.x*3.71+e.posicion.y*1.93).sin()*if e.tipo==EntityType::Boss {0.06}else{0.12}; disparos.push(Entity::enemy_bullet(e.posicion+dir*0.36,Vector2::new((angulo+dispersion).cos(),(angulo+dispersion).sin()))); e.cooldown=if e.tipo==EntityType::Boss {0.95}else{1.65+(e.posicion.x*0.31).rem_euclid(0.85)}; } } }
                entidades.extend(disparos);
                for i in 0..entidades.len() { let tipo=entidades[i].tipo; if !entidades[i].active || !matches!(tipo,EntityType::Bullet|EntityType::EnemyBullet) {continue;} let velocidad=if tipo==EntityType::Bullet {VELOCIDAD_BALA}else{VELOCIDAD_BALA_ENEMIGA}; let direccion=entidades[i].direccion; entidades[i].posicion+=direccion*velocidad*dt; entidades[i].vida-=dt; let pos=entidades[i].posicion; if entidades[i].vida<=0.0 || Map::es_pared(mapa.celda_en(pos.x,pos.y)) {entidades[i].active=false;continue;} if tipo==EntityType::Bullet { let golpe=entidades.iter().enumerate().find_map(|(j,e)|(j!=i&&e.active&&es_combatiente(e.tipo)&&e.posicion.distance_to(pos)<if e.tipo==EntityType::Boss {RADIO_ENEMIGO*1.55}else{RADIO_ENEMIGO}).then_some(j)); if let Some(j)=golpe {entidades[i].active=false; if entidades[j].tipo==EntityType::Boss {entidades[j].vida-=1.0; if entidades[j].vida<=0.0 {entidades[j].active=false;}}else{entidades[j].active=false;} } } else if jugador.posicion.distance_to(pos)<RADIO_ENEMIGO {entidades[i].active=false;jugador.vida=(jugador.vida-DANO_ENEMIGO).max(0);sonido_dano.play();} }
                if jugador.vida<=0 {estado=EstadoPantalla::Derrota;rl.enable_cursor();} else if !entidades.iter().any(|e|e.active&&es_combatiente(e.tipo)) {estado=if nivel==1 {EstadoPantalla::NivelCompletado}else{EstadoPantalla::Victoria};rl.enable_cursor();}
            }
        }
        let impactos=lanzar_rayos(&mapa,jugador.posicion,jugador.angulo); let mut d=rl.begin_drawing(&thread);
        let textura_muro = if nivel == 2 { &textura_muro_nivel2 } else { &textura_muro_nivel1 };
        dibujar_frame(&mut d,true,&mapa,&jugador,&entidades,&impactos,&textura_jugador,&textura_enemigo,textura_muro,&textura_arma,&textura_fuego,estado,opcion_menu,opcion_nivel,opcion_pausa);
    }
}
