use std::io::{self, Write};
const N: usize= 100;

struct Cadena {
    longitud: usize,
    caracteres: [char; N], //Arreglo
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

    //Metodo para invertir la cadena.
    fn invertir(&mut self) {
        if self.longitud <= 1 {
            return
        }
        let mut izq = 0;
        let mut der = self.longitud - 1;
        while izq < der {
            let temp = self.caracteres[izq];
            self.caracteres[izq] = self.caracteres[der];
            self.caracteres[der] = temp;
            izq += 1;
            der -= 1;
        }
    }

    //Metodo para contar vocales y consonantes
    fn contar_vocales_consonantes(&self) -> (usize, usize) {
        let mut vocal:usize = 0;
        let mut consonante: usize = 0;
        for i in 0..self.longitud {
            let car = self.caracteres[i];
            let letra = (car >= 'a' && car <= 'z') || (car >= 'A' && car <= 'Z');
            if letra {
                let esvocal = car == 'a' || car == 'e' || car == 'i' || car == 'o' || car == 'u';
                if esvocal {
                    vocal += 1;
                } else {
                    consonante += 1;
                }
            }
        }
        (vocal, consonante)
    }

    //Metodo para eliminar caracteres duplicados contiguos (consecutivos). Ej:
    //aaabbbccdfd = abcdfd
    fn eliminar_repetidos_consecutivos(&self) -> Cadena {
        let mut cad = Cadena::new();
        for i in 0..self.longitud {
            if i == 0 || self.caracteres[i] != self.caracteres[i-1] {
                cad.add_char(self.caracteres[i]);
            }
        }
        cad
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
    println!("║  6. Invertir cadena              ║");
    println!("║  7. Nros. Vocales y Consonantes  ║");
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

    let mut c = Cadena::new(); //Creando la instancia de clase

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
                    println!("La cadena esta vacia");
                } else {
                    c.invertir();
                    println!("La cadena invertida es: ");
                    c.mostrar();
                }
            }

            "7" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia");
                } else {
                    let (vocal, consonante) = c.contar_vocales_consonantes();
                    println!("El nro de vocales, es: {}", vocal);
                    println!("El nro de consonantes es: {}", consonante);
                }
            }

            "q" | "Q" => { println!("\n  Hasta luego.\n"); break; }
            _          => println!("  Opción no válida."),
        }
    }
}
