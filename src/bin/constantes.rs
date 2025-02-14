const UMA_HORA_EM_SEGUNDOS:i32 =  1 * 60 * 60;


//tipos numéricos
const INTEIRO: i32 = 42;
const UNSIGNED: u32 = 100;
const FLOAT: f64 = 3.14159;
const BYTE: u8 = 255;

//tipo booleano
const VERDADEIRO: bool = true;
const FALSO: bool = false;


//tipos caracteres/string
const CARACTERE: char = 'A';
const TEXTO: &str = "Olá, Rust!";


//arrays
const MESES: [&str; 12] = [
    "Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho",
    "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"
];

const NUMEROS: [i32; 5] = [1, 2, 3, 4, 5];


//tuplas
const PONTO: (i32, i32) = (10, 20);
const CONFIG: (&str, i32, bool) = ("versão", 42, true);

//tempo e duração
const TIMEOUT_SEGUNDOS: u64 = 30;

fn main(){

    println!("INTEIRO: {}", INTEIRO);
    println!("UNSIGNED: {}",UNSIGNED);
    println!("FLOAT: {}",FLOAT);
    println!("BYTE: {} \n",BYTE);

    println!("VERDADEIRO: {}",VERDADEIRO);
    println!("FALSO: {}\n",FALSO);

    println!("CARACTERE: {}",CARACTERE);
    println!("TEXTO: {}\n",TEXTO);

    println!("NUMEROS: {:?}",NUMEROS);
    println!("MESES: {:?}\n",MESES);

    println!("PONTO: {:?}",PONTO);
    println!("CONFIG: {:?}\n",CONFIG);

    println!("TIMEOUT_SEGUNDOS: {:?}",TIMEOUT_SEGUNDOS);

}


    
   