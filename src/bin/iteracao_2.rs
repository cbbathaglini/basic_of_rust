// DESCRIÇÃO
// Percorrendo vetor com for

fn main(){
    
    let array = [10, 20, 30, 40, 50];
    println!("Array: {:?}",array);
    
    percorrendo_array_for(array);
}

fn percorrendo_array_for(array:[i32;5]){

    for elemento in array{
        println!("O valor é: {}", elemento);
    }

}
