use std::io::{self, Write};
const N: usize= 100;

struct Cadena {
    longitud: usize,
    caracteres: [char; N],
}

impl Cadena {
    //Constructor
    fn new() -> Self {
        Cadena {
            longitud: 0,
            caracteres: ['\0'; N],
        }
    }

    fn obtener_longitud(&self) -> usize {
        self.longitud
    }

    //Metodo para adicionar caracteres
    fn add_char(&mut self, c:char) {
        if self.longitud < N {
            self.caracteres[self.longitud] = c;
            self.longitud += 1;
        }
    }

    //Metodo para devolver un caracter, dada la posicion.
    fn obtener_char(&self, pos: usize) -> char {
        if pos > 0 && pos <= self.longitud {
            self.caracteres[pos-1]
        } else {
            '\0'
        }
    }

    //Metodo para contar la cantidad de apariciones de un caracter.
     fn contar_apariciones(&self, c:char) -> usize {
        let mut contador: usize = 0;
        for i in 0..self.longitud {
            if self.caracteres[i] == c {
                contador += 1;
            }
        }
        contador
    }

    //Metodo que devuelva el caracter mas repetido.
     fn char_mas_repetido(&self) -> char {
        let mut max_char = self.caracteres[0];
        let mut max_cont = 0;

        for i in 0..self.longitud {
            let car = self.caracteres[i];
            let mut cont = 0;
            for j in 0..self.longitud {
                if self.caracteres[j] == car {
                    cont += 1;
                }
            }
            if cont > max_cont {
                max_cont = cont;
                max_char = car;
            }
        }
        max_char
    }

    fn limpiar(&mut self) {
        self.longitud = 0;
        self.caracteres = ['\0'; N];
    }

    fn mostrar(&self) {
        for i in 0..self.longitud {
            print!("{}", self.caracteres[i]);
        }
        println!();
    }
}

fn leer_linea() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}

fn leer_numero() -> Option<usize> {
    leer_linea().parse::<usize>().ok()
}

fn mostrar_menu(c: &Cadena) {
    // construimos la cadena actual para mostrarla en el encabezado
    let mut preview = String::new();
    for i in 0..c.longitud {
        preview.push(c.caracteres[i]);
    }
    if preview.is_empty() {
        preview = String::from("(vacía)");
    }

    println!("\n╔══════════════════════════════════╗");
    println!("║   CADENA: {:>22}  ║", preview);
    println!("╠══════════════════════════════════╣");
    println!("║  1. Ingresar nueva cadena        ║");
    println!("║  2. Mostrar cadena               ║");
    println!("║  3. Longitud                     ║");
    println!("║  4. Obtener carácter (posición)  ║");
    println!("║  5. Cantidad repeticiones (char) ║");
    println!("║  6. Caracter + repetido          ║");
    println!("╠══════════════════════════════════╣");
    println!("║  Q. Salir                        ║");
    println!("╚══════════════════════════════════╝");
    print!("   Opción: ");
    io::stdout().flush().expect("Error al mostrar menú");
}

fn main() {
    println!("════════════════════════════════════");
    println!("  Cadenas - POO — Programación I   ");
    println!("════════════════════════════════════");

    let mut c = Cadena::new(); //Creando la insancia de clase

    loop {
        mostrar_menu(&c);
        let opcion = leer_linea();

        match opcion.as_str() {
            "1" => {
                println!("  Ingresa la cadena:");
                let entrada = leer_linea();

                c.limpiar(); // reiniciamos antes de cargar la nueva

                // ── proceso artesanal: carácter por carácter ──
                for ch in entrada.chars() {
                    c.add_char(ch);
                }

                println!("  ✓ Cadena cargada ({} caracteres)", c.obtener_longitud());
            }

            "2" => {
                print!("  Cadena: ");
                c.mostrar();
            }

            "3" => println!("  Longitud: → {}", c.obtener_longitud()),

            "4" => {
                println!("  Ingresa la posición (1 = izquierda):");
                match leer_numero() {
                    Some(pos) if pos >= 1 && pos <= c.obtener_longitud() => {
                        println!("  Carácter en posición {}: → '{}'", pos, c.obtener_char(pos));
                    }
                    Some(_) => println!("  Posición fuera de rango (1 a {}).", c.obtener_longitud()),
                    None    => println!("  Posición inválida."),
                }
            }

            "5" => {
                println!("  Ingresa el caracter:");
                let entrada = leer_linea();
                match entrada.chars().next() {
                    Some(car)  => {
                        let cantidad = c.contar_apariciones(car);
                        println!("  El caracter aparecer: {} vez/veces", cantidad);
                    }
                    None    => println!("  No ingresaste ningun caracter choquito."),
                }
            }
            "6" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia choquito.");
                } else {
                    let resultado = c.char_mas_repetido();
                    println!("El caracter que mas se repite es: {}", resultado);
                }
            }

            "q" | "Q" => { println!("\n  Hasta luego.\n"); break; }
            _          => println!("  Opción no válida."),
        }
    }
}
