struct Numero {
    valor: u64
}

impl Numero {
    //Constructor
    fn new(valor: u64) -> Self {
        Numero { valor }
    }

    fn es_par(&self) -> bool {
        self.valor % 2 == 0
    }

    //fn es_par(x: u64) -: bool 
    //     x % 2 == 0;

    fn cantidaddigitos(&self) -> u64 {
        let mut count: u64 = 0;
        let mut num: u64 = self.valor;

        while num > 0 {
            num /= 10;
            count += 1;
        }

        count
    }

    //Método para eliminar un dígito, dado el dígito, no la posición.
    fn dev_pos_dig(&self, digito: u64) -> u64{
        let mut num = self.valor;
        let mut pos = self.cantidaddigitos();
        while num > 0 {
            let dig = num%10;
            if dig == digito {
               return pos; 
            }
            num /= 10;
            pos -= 1;
        }        
        0
    }

    fn eliminar_dig(&mut self, digito: u64) {
        let pos = self.dev_pos_dig(digito);
        if pos == 0 {
            println!("No se encontró el digito.");
            return;
        }
        self.eliminar(pos);
    } 

    fn eliminar(&mut self, pos: u64) {
        let total = self.cantidaddigitos() as u64;


        let mut peso: u64 = 1;
        for _ in 0..(total - pos) {
            peso *= 10;
        }

        let parte_izquierda = self.valor / (peso * 10);  
        let parte_derecha   = self.valor % peso;        
        self.valor = parte_izquierda * peso + parte_derecha;
    }

    fn binario(&self) -> u64  {
        let mut num = self.valor;
        let mut bin = 0;
        let mut mult = 1;
        while num > 0 {
            let residuo = num % 2;
            bin += residuo * mult;
            mult *= 10;
            num /= 2;
        }
        bin
    }

    fn hexadecimal(&self) ->String  { //Hexadecimal = /16
        let mut num = self.valor;
        let mut resultado = String::new();
        let dig = b"0123456789ABCDEF"; //b significa arreglo.
        while num>0  {
            let residuo = (num%16) as usize;
            resultado.insert(0, dig[residuo] as char);
            num /= 16;
        }
        resultado
    }
}

fn main() {
    //funcion main
    println!("=============================");
    println!("Struct Numero");
    println!("=============================");
    //Crear instancia del objeto Numero: n
    let mut n = Numero::new(34); //57104 //7104

    println!("El valor de la instancia n es: {}", n.valor);
    println!("El numero en binario es: {}", n.binario());
    //n.eliminar(5);
    //println!("Eliminando el digito en la posicion 5 es: {}", n.valor);
    println!("La posicion del digito 5 es: {}",n.dev_pos_dig(5));

    //println!("El valor n es par?: {}", n.es_par());
    n.eliminar_dig(5);

    println!("Eliminando el digito 5: {}", n.valor);

    //println!("El valor n es par?: {}", n.multiplicar(3));
}
