const MAX: usize = 100;

struct Vectores {
    //Atributos
    dimension: usize,
    elemento: [u64;MAX],
}

impl Vectores {
    //Constructor
    fn new() -> Vectores {
        Vectores {
            dimension: 0,
            elemento: [0;MAX],
        }
    }

    fn dim(&self) -> usize {
        self.dimension
    }

    fn dimensionar(&mut self, d:usize) {
        self.dimension = d;
    }

    fn addelement(&mut self, e:u64) {
        if self.dimension < MAX {
            self.elemento[self.dimension] = e;
            self.dimension += 1;
        }
    }

    fn show(&self) {
        for i in 0..self.dimension { //Va hasta el nro - 1
            println!("Elemento[{}] = {}", i, self.elemento[i]);
        }
    }

    fn reemplazar(&mut self, p:usize, e:u64){
        if p <= self.dimension {
            self.elemento[p-1] = e;
        }
    }

    fn insertar(&mut self, p:usize, e:u64){
        if (self.dimension < MAX) && (p <= self.dimension) {
            self.dimension += 1;
            let mut x = self.dimension;
            while x > p-1 {
                self.elemento[x-1] = self.elemento[x-2];
                x -= 1;
            }
            self.elemento[p-1] = e;
        }
    }
}

fn main() {
    //Crear instancia
    let mut v = Vectores::new();
    v.addelement(9);
    v.addelement(2);
    v.addelement(1);
    v.addelement(0);
    v.addelement(7);
    v.addelement(19);
    v.addelement(8);
    v.addelement(3);

    println!("----------------------");
    v.show();
    println!("----------------------");
    println!("La cantidad de elementos es: {}", v.dim());

    v.insertar(7, 5);
    println!("----------------------");
    v.show();
    println!("----------------------");
    println!("La cantidad de elementos es: {}", v.dim());
}
