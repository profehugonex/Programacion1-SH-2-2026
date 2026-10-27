const NF:usize = 50;
const NC:usize = 50;

struct Matriz{
    nrofilas: usize,
    nrocol: usize,
    celda:[[u64; NC];NF],
}

impl Matriz{
    //Constructor
    fn new(nrofilas:usize, nrocol:usize) ->Matriz{
        Matriz {
            nrofilas,
            nrocol,
            celda:[[0;NC];NF],
        }
    }

    fn obtfilas(&self)-> usize{
        self.nrofilas
    }

    fn obtcol(&self) -> usize{
        self.nrocol
    }

    fn get_celda(&self,f:usize, c:usize) ->u64{
        self.celda[f][c]
    }

    fn set_celda(&mut self, f:usize, c:usize, valor:u64){
        self.celda[f][c] = valor;
    }
}

fn main() {
    println!("Hello, world!");
}
