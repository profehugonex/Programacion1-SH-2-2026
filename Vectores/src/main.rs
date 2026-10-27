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

    fn  pares_impares(&self)  -> bool {
        //Metodo para devolver si exsisten mas nrs pares o mas nrs impares.
        let mut par = 0;
        let mut impar = 0;
        for i in 0..self.dimension {
            if self.elemento[i]%2 == 0 {
                par += 1;
            } else {
                impar += 1;
            }
        }
        par > impar
    }

    fn insertion_sort(&mut self) {
        for i in 1..self.dimension {
            let llave = self.elemento[i];
            let mut j = i;

            while j > 0 && self.elemento[j-1] > llave {
                self.elemento[j] = self.elemento[j-1];
                j -=1 ;
            }
            self.elemento[j] = llave;
        }
    }

    fn merge_sort(&mut self) {
        if self.dimension <= 1 { return }
        self.merge_sort_rango(0, self.dimension-1);
    }

    fn merge_sort_rango(&mut self, izq: usize, der:usize) {
        //recursividad
        if izq >= der { return }
        
        let medio = (izq + der) / 2;
        self.merge_sort_rango(izq, medio);
        self.merge_sort_rango(medio+1, der);

        self.merge(izq, medio, der);
    }

    fn merge(&mut self, izq: usize, medio: usize, der: usize) {
        let mut derar = [0_u64; 100];
        let mut izqar = [0_u64; 100];

        let tam_izq = medio - izq + 1;
        let tam_der = der - medio;

        for i in 0..tam_izq {
            izqar[i] = self.elemento[izq + i];
        }

        for i in 0..tam_der {
            derar[i] = self.elemento[medio + 1 + i];
        }

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = izq;

        while i < tam_izq && j < tam_der {
            if izqar[i] <= derar[j] {
                self.elemento[k] = izqar[i];
                i += 1;
            } else {
                self.elemento[k] = derar[j];
                j += 1;
            }
            k += 1;
        }

        while i < tam_izq {
            self.elemento[k] = izqar[i];
            i += 1;
            k += 1;
        }

        while j < tam_der {
            self.elemento[k] = derar[j];
            j += 1;
            k += 1;
        }
    }


    //Metodo de ordenamiento: QickSort (Ordenamiento Rápido)
    fn quick_sort(&mut self){
        if self.dimension <= 1 { return }
        self.quicksort_rango(0,self.dimension-1);
    }

    fn quicksort_rango(&mut self, izq:usize, der:usize){
        if izq >= der { return; }

        let pivote_pos = self.particionar(izq, der);
        if pivote_pos > 0 {
            self.quicksort_rango(izq, pivote_pos-1);
        }
        self.quicksort_rango(pivote_pos+1, der);
    }

    fn particionar(&mut self, izq:usize, der:usize) -> usize{
        let pivote = self.elemento[der];
        let mut i = izq;
        for j in izq..der{
            if self.elemento[j] <= pivote{
                let temp = self.elemento[i];
                self.elemento[i] = self.elemento[j];
                self.elemento[j] = temp;
                i += 1;
            }
        }
        let temp = self.elemento[i];
        self.elemento[i] = self.elemento[der];
        self.elemento[der] = temp;
        i
    }

   /* fn escalera(&self) -> bool {
        //Metodo para devolver true o false, si es que existe una escalera en el vector.
    }*/
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
    println!("----------------------");
    
    if v.pares_impares() {
        println!("Hay mas nros PARES en el vector");
    } else {
        println!("Hay mas nrs IMPARES en el vector");
    }

    println!(" "); 
    println!("----------------------"); 
     println!("---------SORT--------");
     v.merge_sort();
    v.show();
    println!("----------------------");    
}
