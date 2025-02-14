// DESCRIÇÃO
// Percorrendo vetor dinâmico com for

fn main(){
    
    let mut numeros: Vec<i32> = Vec::new();

    numeros.push(20);
    numeros.push(10);
    numeros.push(3);
    numeros.push(7);

    percorrendo_array_for(numeros);
    
}

fn percorrendo_array_for(numeros:Vec<i32>){

    for numero in &numeros{
        println!("O valor é: {}", numero);
    }
    
}
