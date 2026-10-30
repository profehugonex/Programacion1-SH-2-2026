const NF: usize = 100;
const NC: usize = 100;

struct Matriz{
    nro_filas: usize,
    nro_col: usize,
    celda: [[u64;NC];NF],
}

impl Matriz{
    //Constructor
    fn new(nro_filas:usize, nro_col:usize) ->Matriz{
        Matriz{
            nro_filas,
            nro_col,
            celda: [[0;NC];NF],
        }
    }

    fn obt_nrofilas(&self) ->usize {
        self.nro_filas
    }

    fn obt_nrocol(&self) ->usize{
        self.nro_col
    }

    fn get_celda(&self, f:usize, c:usize) ->u64{
        self.celda[f][c]
    }

    fn set_celda(&mut self, f:usize, c:usize, valor:u64){
        self.celda[f][c] = valor;
    }

    //Metodo para sumar todos los elementos de una matriz.
    fn suma(&self) ->u64{
        let mut suma = 0;
        for i in 0..self.nro_filas{
            for j in 0..self.nro_col{
                suma = suma + self.celda[i][j];
            }
        }
        suma
    }

    //Metodo para contar los nros pares que tiene la matriz.
    fn cant_pares(&self) ->u16{
        let mut pares = 0;
        for i in 0..self.nro_filas{
            for j in 0..self.nro_col{
                if self.celda[i][j]%2 == 0{
                    pares += 1;
                }
            }
        }
        pares
    }

    fn impares(&self) ->u16{
        let mut impares = 0;
        for i in 0..self.nro_filas{
            for j in 0..self.nro_col{
                if self.celda[i][j]%2 != 0{
                    impares += 1;
                }
            }
        }
        impares
    }

    fn mostrar(&self){
        println!();
        for i in 0..self.nro_filas{
            println!(" ");
            for _ in 0..self.nro_col{
                print!("┌─────────┐");
            }
            println!();
            print!("  ");
            for c in 0..self.nro_col {
                print!("│{:^9}│", self.celda[i][c]);
            }
            println!();
            print!("  ");
            for _ in 0..self.nro_col {
                print!("└─────────┘");
            }
            println!();
        }
        print!("  ");
        for c in 0..self.nro_col {
            print!("{:^11}", format!("c{}", c));
        }
        println!("\n");
    }
}




fn main() {
    println!("════════════════════════════════════════");
    println!("  Matrices - POO — Programación I      ");
    println!("════════════════════════════════════════");

    let mut m = Matriz::new(3, 3);

    m.celda[0][0] = 1;   m.celda[0][1] = 2;   m.celda[0][2] = 3;
    m.celda[1][0] = 14;   m.celda[1][1] = 5;   m.celda[1][2] = 6;
    m.celda[2][0] = 7;   m.celda[2][1] = 8;   m.celda[2][2] = 9;

    println!("Matriz original:");
    m.mostrar();
    println!("════════════════════════════════════════");
    println!("La suma de todos los valores es: {}", m.suma());
    println!("════════════════════════════════════════");
    println!("La cantidad de pares es: {}", m.cant_pares());
    println!("La cantidad de impares es: {}", m.impares());
}
