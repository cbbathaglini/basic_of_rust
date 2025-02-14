// DESCRIÇÃO
// Verificando se é par ou ímpar

fn main(){
    
    eh_par(10);
    eh_par(11);
    eh_par(36);
    eh_par(167);
     
}


fn eh_par(numero:i32){
    if numero % 2 == 0 {
        println!("{numero} eh par!");
    }else{
        println!("{numero} eh ímpar!");
    }
}
