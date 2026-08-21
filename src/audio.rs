use raylib::prelude::*;

use crate::framebuffer::EstadoPantalla;

/// Recursos y comportamiento de audio del juego.
/// El dispositivo se mantiene en `main` para que sus recursos prestados vivan
/// durante toda la ejecución.
pub struct AudioJuego<'a> {
    disparo: Sound<'a>,
    dano: Sound<'a>,
    recarga: Sound<'a>,
    musica_nivel1: Option<Music<'a>>,
    musica_nivel2: Option<Music<'a>>,
    musica_menu: Option<Music<'a>>,
}

impl<'a> AudioJuego<'a> {
    pub fn cargar(audio: &'a RaylibAudio) -> Self {
        let disparo = cargar_sonido(audio, &crear_disparo(), "disparo");
        let dano = cargar_sonido(audio, &crear_onda(96.0, 0.22, true), "dano");
        let recarga = cargar_sonido(audio, &crear_onda(310.0, 0.14, false), "recarga");
        let musica_nivel1 = cargar_musica(audio, "assets/music/Taylor Swift - Style.mp3", 0.32);
        let musica_nivel2 = cargar_musica(audio, "assets/music/Taylor Swift - Blank Space.mp3", 0.30);
        let musica_menu = cargar_musica(audio, "assets/music/Taylor Swift - Look What You Made Me Do.mp3", 0.24);
        Self { disparo, dano, recarga, musica_nivel1, musica_nivel2, musica_menu }
    }

    pub fn actualizar(&mut self, estado: EstadoPantalla, nivel: u8) {
        actualizar_musica(&self.musica_nivel1, estado == EstadoPantalla::Jugando && nivel == 1);
        actualizar_musica(&self.musica_nivel2, estado == EstadoPantalla::Jugando && nivel == 2);
        actualizar_musica(&self.musica_menu, matches!(estado, EstadoPantalla::Bienvenida | EstadoPantalla::SeleccionNivel | EstadoPantalla::Instrucciones));
    }

    pub fn reproducir_disparo(&self) { self.disparo.play(); }
    pub fn reproducir_dano(&self) { self.dano.play(); }
    pub fn reproducir_recarga(&self) { self.recarga.play(); }
}

fn cargar_sonido<'a>(audio: &'a RaylibAudio, datos: &[u8], nombre: &str) -> Sound<'a> {
    let onda = audio.new_wave_from_memory(".wav", datos).unwrap_or_else(|_| panic!("No se pudo crear el sonido de {nombre}"));
    audio.new_sound_from_wave(&onda).unwrap_or_else(|_| panic!("No se pudo cargar el sonido de {nombre}"))
}

fn cargar_musica<'a>(audio: &'a RaylibAudio, ruta: &str, volumen: f32) -> Option<Music<'a>> {
    let musica = audio.new_music(ruta).ok();
    if let Some(musica) = &musica { musica.set_volume(volumen); }
    musica
}

fn actualizar_musica(musica: &Option<Music<'_>>, debe_sonar: bool) {
    if let Some(musica) = musica {
        if debe_sonar {
            if !musica.is_stream_playing() { musica.play_stream(); }
            musica.update_stream();
        } else if musica.is_stream_playing() {
            musica.stop_stream();
        }
    }
}

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
    for i in 0..muestras { let t = i as f32 / tasa as f32; semilla = semilla.wrapping_mul(1_664_525).wrapping_add(1_013_904_223); let ruido = ((semilla >> 9) as f32 / 8_388_608.0) - 1.0; let fogonazo = (-t * 32.0).exp(); let chasquido = ruido * (-t * 24.0).exp() * 0.70; let golpe = (t * (105.0 - t * 300.0).max(35.0) * std::f32::consts::TAU).sin() * fogonazo * 0.52; let cola = ruido * (-(t - 0.035).max(0.0) * 18.0).exp() * 0.12; let muestra = ((chasquido + golpe + cola) * i16::MAX as f32 * 0.72).clamp(i16::MIN as f32, i16::MAX as f32) as i16; wav.extend_from_slice(&muestra.to_le_bytes()); }
    wav
}
