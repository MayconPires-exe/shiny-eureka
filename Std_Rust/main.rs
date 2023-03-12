const PI:f32 = 3.1415;
static mut GLOBAL:u8 = 1;

fn escopo() {
  println!("PI = {}. tamanho = {} bytes", PI, std::mem::size_of_val(&PI));

  unsafe {
    println!("GLOBAL = {}. tamanho = {} bytes", GLOBAL, std::mem::size_of_val(&GLOBAL));
  }

  let variavel:i32 = 300;
  println!("variavel = {}. tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

  let decimal:f32 = 3.1;
  println!("decimal = {}. tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));

  let booleano:bool = true;
  println!("booleano = {}. tamanho = {} bytes", booleano, std::mem::size_of_val(&booleano));

  let caracter:char = 'C';
  println!("caracter = {}. tamanho = {} bytes", caracter, std::mem::size_of_val(&caracter));
}

fn main() {
  escopo();
}