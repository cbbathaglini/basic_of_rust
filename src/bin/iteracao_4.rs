// DESCRIÇÃO
// Percorrendo vetor dinâmico com for

fn main(){
    
    println!("Percorrendo: ");
    percorrendo_array_for();


    println!("\nPercorrendo reverso: ");
    percorrendo_array_for_reverse();
    
}

fn percorrendo_array_for(){

    for numero in (1..5){
        println!("O valor é: {}", numero);
    }
    
}

fn percorrendo_array_for_reverse(){

    for numero in (1..5).rev(){
        println!("O valor é: {}", numero);
    }
    
}
