# Luis Alejandro Hernández Márquez (241424)
# Gráficas por computadora 
# Prof. Pablo Koch
# Wicho Slug: Operación Bunker

Juego de disparos en primera persona creado en Rust con raylib. El jugador recorre un búnker mediante *raycasting*, recoge equipo y combate soldados enemigos hasta derrotar al jefe final.

## Características

- Dos niveles seleccionables desde el menú.
- Renderizado pseudo-3D por raycasting, con texturas de muros y cielo por nivel.
- Enemigos, proyectiles, botiquines, munición, arma recogible y jefe final.
- HUD de vida, munición, enemigos y jefe.
- Menú principal, selección de misión, instrucciones, pausa, derrota y victoria.
- Sonidos generados en tiempo de ejecución para disparo, daño y recarga; música independiente para menú y niveles.
- Soporte de teclado, mouse y control compatible.

## Requisitos

- [Rust] estable, con Cargo.
- Un entorno capaz de ejecutar aplicaciones gráficas de raylib.

La dependencia `raylib` se descarga y compila automáticamente con Cargo.

## Ejecutar

Desde la raíz del proyecto:

```bash
cargo run
```

Para comprobar que el proyecto compila sin abrir el juego:

```bash
cargo check
```

## Video de jugabilidad

[video](https://youtu.be/z0nF7lbERTI)

## Controles

| Acción | Teclado / mouse | Control |
| --- | --- | --- |
| Moverse | `WASD` o flechas | Stick izquierdo |
| Girar | Mouse, `Q` / `E` o flechas izquierda/derecha | Stick derecho |
| Disparar | Clic izquierdo o `Espacio` | — |
| Recargar | `R` | — |
| Pausar / reanudar | `P` o `F1` | — |
| Navegar menús | Flechas y `Enter` | — |
| Volver al menú desde instrucciones | `Enter` o `M` | — |
| Reintentar tras derrota / volver tras victoria | `R` | — |
| Salir | `Esc` o la opción del menú | — |

## Objetivo

En cada misión recoge el arma, munición y botiquines mientras eliminas a los enemigos. Al completar el Nivel 1 se desbloquea el paso al Nivel 2. La victoria se obtiene al derrotar al jefe final del Nivel 2.

## Música

| Escena | Canción |
| --- | --- |
| Menú e instrucciones | Taylor Swift — *Look What You Made Me Do* |
| Nivel 1: Búnker | Taylor Swift — *Style* |
| Nivel 2: Jefe final | Taylor Swift — *Blank Space* |

Los archivos de música y las texturas se encuentran en `assets/`.

Los mapas usan los siguientes símbolos: `P` para aparición del jugador, `G` arma, `M` munición, `H` botiquín, `F` fuego, `E` enemigo y `B` jefe.
