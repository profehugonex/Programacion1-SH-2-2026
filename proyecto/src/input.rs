use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum Tecla {
    FlechaIzquierda,
    FlechaDerecha,
    Espacio,
    Q,
    Otro,
}

pub fn leer_tecla() -> Option<Tecla> {
    // poll con timeout de 0ms → no bloqueante
    if event::poll(Duration::from_millis(0)).unwrap_or(false) {
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            return match code {
                KeyCode::Left  => Some(Tecla::FlechaIzquierda),
                KeyCode::Right => Some(Tecla::FlechaDerecha),
                KeyCode::Char(' ') => Some(Tecla::Espacio),
                KeyCode::Char('q') | KeyCode::Char('Q') => Some(Tecla::Q),
                _ => Some(Tecla::Otro),
            };
        }
    }
    None
}
