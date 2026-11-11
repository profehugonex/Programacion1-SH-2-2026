mod input;
mod entidades;

use input::{leer_tecla, Tecla};
use entidades::Juego;
use crossterm::terminal;

fn main() {
    terminal::enable_raw_mode().expect("No se pudo activar raw mode");

    println!("SPACE INVADERS - ← → Mover | ESPACIO Disparar | Q Salir");
    std::thread::sleep(std::time::Duration::from_secs(2));

    let mut juego = Juego::new();

    loop {
        juego.actualizar();
        juego.dibujar();

        if let Some(tecla) = leer_tecla() {
            juego.procesar_tecla(tecla);
        }

        std::thread::sleep(std::time::Duration::from_millis(150));
    }
}
