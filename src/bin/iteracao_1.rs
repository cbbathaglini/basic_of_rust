// DESCRIÇÃO
// Percorrendo vetor com loop

fn main(){
    
    let array = [10, 20, 30, 40, 50];
    println!("Array: {:?}",array);
    
    percorrendo_array_loop(array);
}

fn percorrendo_array_loop(array:[i32;5]){
    let mut indice = 0;
    while indice < 5 {
        println!("O valor é: {}", array[indice]);

        indice = indice + 1;
    }
}
