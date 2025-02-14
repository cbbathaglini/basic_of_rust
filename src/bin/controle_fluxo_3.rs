// DESCRIÇÃO
// Verificando se é par ou ímpar e retornando true ou false

fn main(){
    
    println!("10 eh par? {}",eh_par(10));
    println!("11 eh par? {}",eh_par(11));
    println!("36 eh par? {}",eh_par(36));
    println!("167 eh par? {}",eh_par(167));
     
}

fn eh_par(numero:i32) -> bool{
    if numero % 2 == 0 {
        true
    }else{
        false
    }
}
