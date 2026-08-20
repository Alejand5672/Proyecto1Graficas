mod config; mod entity; mod framebuffer; mod map; mod player; mod raycasting;
use config::{DANO_ENEMIGO, DISTANCIA_ATAQUE_ENEMIGO, ENFRIAMIENTO_DISPARO, RADIO_ENEMIGO, RADIO_PICKUP, TAM_CELDA, VELOCIDAD_BALA, VELOCIDAD_BALA_ENEMIGA};
use entity::{Entity, EntityType}; use framebuffer::{dibujar_frame, EstadoPantalla}; use map::Map; use player::Player; use raycasting::{lanzar_rayo, lanzar_rayos}; use raylib::prelude::*;

fn cargar_nivel(nivel: u8) -> (Map, Vector2, Vec<Entity>) { Map::cargar(if nivel == 1 { "laberinto1.txt" } else { "laberinto2.txt" }) }
fn es_combatiente(tipo: EntityType) -> bool { matches!(tipo, EntityType::Enemy | EntityType::Boss) }

fn main() {
    let (mut mapa, mut spawn, mut entidades) = cargar_nivel(1);
    let (mut rl, thread) = raylib::init().size(mapa.columnas as i32 * TAM_CELDA, mapa.filas as i32 * TAM_CELDA).title("Wicho Slug").build();
    rl.set_target_fps(60);
    let textura_jugador = rl.load_texture(&thread, "assets/player.png").expect("No se pudo cargar assets/player.png");
    let textura_enemigo = rl.load_texture(&thread, "assets/enemy_soldier.png").expect("No se pudo cargar assets/enemy_soldier.png");
    let mut textura_muro = rl.load_texture(&thread, "assets/wall_bunker.png").expect("No se pudo cargar assets/wall_bunker.png"); textura_muro.gen_texture_mipmaps(); textura_muro.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_TRILINEAR);
    let textura_arma = rl.load_texture(&thread, "assets/first_person_hands.png").expect("No se pudo cargar assets/first_person_hands.png");
    let mut jugador = Player::new(spawn); let mut nivel = 1_u8; let mut estado = EstadoPantalla::Bienvenida; let mut vista_3d = true; let mut tiempo_disparo = 0.0_f32; rl.enable_cursor();
    while !rl.window_should_close() {
        let dt = rl.get_frame_time().min(0.05);
        match estado {
            EstadoPantalla::Bienvenida => {
                if rl.is_key_pressed(KeyboardKey::KEY_ONE) || rl.is_key_pressed(KeyboardKey::KEY_LEFT) { nivel = 1; }
                if rl.is_key_pressed(KeyboardKey::KEY_TWO) { nivel = 2; }
                if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) { nivel = 2; }
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    (mapa, spawn, entidades) = cargar_nivel(nivel); jugador = Player::new(spawn); tiempo_disparo = 0.0; estado = EstadoPantalla::Jugando; rl.disable_cursor();
                }
            }
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
                if rl.is_key_pressed(KeyboardKey::KEY_Z) || rl.is_key_pressed(KeyboardKey::KEY_TAB) { vista_3d = !vista_3d; if vista_3d { rl.disable_cursor(); } else { rl.enable_cursor(); } }
                jugador.actualizar(&rl, &mapa, dt, vista_3d); tiempo_disparo = (tiempo_disparo - dt).max(0.0); jugador.retroceso = (jugador.retroceso - dt).max(0.0);
                for e in &mut entidades { if e.active && jugador.posicion.distance_to(e.posicion) < RADIO_PICKUP { match e.tipo { EntityType::Weapon => { e.active=false; jugador.has_weapon=true; jugador.municion+=30; }, EntityType::Ammo => { e.active=false; jugador.municion+=15; }, EntityType::Health => { e.active=false; jugador.vida=(jugador.vida+35).min(100); }, _=>{} } } }
                if jugador.has_weapon && jugador.municion > 0 && tiempo_disparo <= 0.0 && (rl.is_key_pressed(KeyboardKey::KEY_SPACE) || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)) { let dir=Vector2::new(jugador.angulo.cos(),jugador.angulo.sin()); entidades.push(Entity::bullet(jugador.posicion+dir*0.28,dir)); jugador.municion-=1; tiempo_disparo=ENFRIAMIENTO_DISPARO; jugador.retroceso=0.13; }
                let mut disparos=Vec::new();
                for e in entidades.iter_mut().filter(|e| e.active && es_combatiente(e.tipo)) { e.cooldown=(e.cooldown-dt).max(0.0); let hacia=jugador.posicion-e.posicion; let distancia=hacia.length(); if distancia>0.2 && distancia<=DISTANCIA_ATAQUE_ENEMIGO && e.cooldown<=0.0 { let dir=hacia/distancia; let angulo=dir.y.atan2(dir.x); if lanzar_rayo(&mapa,e.posicion,angulo,distancia).distancia>=distancia-0.06 { let dispersion=(e.posicion.x*3.71+e.posicion.y*1.93).sin()*if e.tipo==EntityType::Boss {0.06}else{0.12}; disparos.push(Entity::enemy_bullet(e.posicion+dir*0.36,Vector2::new((angulo+dispersion).cos(),(angulo+dispersion).sin()))); e.cooldown=if e.tipo==EntityType::Boss {0.95}else{1.65+(e.posicion.x*0.31).rem_euclid(0.85)}; } } }
                entidades.extend(disparos);
                for i in 0..entidades.len() { let tipo=entidades[i].tipo; if !entidades[i].active || !matches!(tipo,EntityType::Bullet|EntityType::EnemyBullet) {continue;} let velocidad=if tipo==EntityType::Bullet {VELOCIDAD_BALA}else{VELOCIDAD_BALA_ENEMIGA}; let direccion=entidades[i].direccion; entidades[i].posicion+=direccion*velocidad*dt; entidades[i].vida-=dt; let pos=entidades[i].posicion; if entidades[i].vida<=0.0 || Map::es_pared(mapa.celda_en(pos.x,pos.y)) {entidades[i].active=false;continue;} if tipo==EntityType::Bullet { let golpe=entidades.iter().enumerate().find_map(|(j,e)|(j!=i&&e.active&&es_combatiente(e.tipo)&&e.posicion.distance_to(pos)<if e.tipo==EntityType::Boss {RADIO_ENEMIGO*1.55}else{RADIO_ENEMIGO}).then_some(j)); if let Some(j)=golpe {entidades[i].active=false; if entidades[j].tipo==EntityType::Boss {entidades[j].vida-=1.0; if entidades[j].vida<=0.0 {entidades[j].active=false;}}else{entidades[j].active=false;} } } else if jugador.posicion.distance_to(pos)<RADIO_ENEMIGO {entidades[i].active=false;jugador.vida=(jugador.vida-DANO_ENEMIGO).max(0);} }
                if jugador.vida<=0 {estado=EstadoPantalla::Derrota;rl.enable_cursor();} else if !entidades.iter().any(|e|e.active&&es_combatiente(e.tipo)) {estado=if nivel==1 {EstadoPantalla::NivelCompletado}else{EstadoPantalla::Victoria};rl.enable_cursor();}
            }
        }
        let impactos=lanzar_rayos(&mapa,jugador.posicion,jugador.angulo); let mut d=rl.begin_drawing(&thread);
        dibujar_frame(&mut d,vista_3d,&mapa,&jugador,&entidades,&impactos,&textura_jugador,&textura_enemigo,&textura_muro,&textura_arma,estado,nivel);
    }
}
