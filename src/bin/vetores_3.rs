// DESCRIÇÃO
// Declarando vetor com tamanho dinâmico Vec<i32>
// Iterando no vetor, removedo e adicionando

fn main(){
    
    let numeros = get_numeros(); // Vec<i32>
    println!("\nNumeros: {:?}", numeros);
    
}

fn get_numeros() -> Vec<i32> {
    let mut numeros: Vec<i32> = Vec::new();

    numeros.push(20);
    numeros.push(10);
    numeros.push(3);
    numeros.push(7);

    println!("[1] Numeros: {:?}", numeros);

    numeros.pop();
    println!("[2] Numeros: {:?}", numeros);

    if let Some(numero_escolhido) = numeros.get(1) {
        println!("[3] Numeros posição 0: {:?}", numero_escolhido);
    }

    print!("Iterando: ");
    for numero in &numeros{
        print!("{numero} ");
    }

    numeros
}

