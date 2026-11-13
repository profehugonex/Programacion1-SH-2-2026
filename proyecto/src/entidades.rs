use crate::input::Tecla;
use std::process;

const ANCHO: i32 = 40;
const ALTO: i32 = 15;

pub struct Jugador {
    pub x: i32,
}

impl Jugador {
    pub fn new() -> Jugador {
        Jugador { x: ANCHO / 2 }
    }
    pub fn mover_izquierda(&mut self) {
        if self.x > 0 { self.x -= 1; }
    }
    pub fn mover_derecha(&mut self) {
        if self.x < ANCHO - 1 { self.x += 1; }
    }
}

pub struct Enemigo {
    pub x: i32,
    pub y: i32,
}

impl Enemigo {
    pub fn new(x: i32, y: i32) -> Enemigo {
        Enemigo { x, y }
    }
}

//struct Juego, tiene atributos que son a su vez struct.
pub struct Juego {
    pub jugador: Jugador,
    pub enemigos: Vec<Enemigo>,
}

impl Juego {
    pub fn new() -> Juego {
        let mut enemigos = Vec::new();
        for i in 0..8 {
            enemigos.push(Enemigo::new(5 + i * 4, 1));
        }
        Juego {
            jugador: Jugador::new(),
            enemigos,
        }
    }

    pub fn actualizar(&mut self) {
        // Intencionalmente vacío: enemigos quietos, sin balas.
    }

    pub fn dibujar(&self) {
        print!("\x1B[2J\x1B[H");
        for y in 0..ALTO {
            let mut linea = String::with_capacity(ANCHO as usize);
            for x in 0..ANCHO {
                if self.enemigos.iter().any(|e| e.x == x && e.y == y) {
                    linea.push('W');
                } else if self.jugador.x == x && y == ALTO - 1 {
                    linea.push('A');
                } else {
                    linea.push(' ');
                }
            }
            println!("{}", linea);
        }
        println!("Posición del jugador: x = {}", self.jugador.x);
        println!("← → Mover | Q Salir");
    }

    pub fn procesar_tecla(&mut self, tecla: Tecla) {
        match tecla {
            Tecla::FlechaIzquierda => self.jugador.mover_izquierda(),
            Tecla::FlechaDerecha   => self.jugador.mover_derecha(),
            Tecla::Q               => process::exit(0),
            _                      => {}
        }
    }
}
