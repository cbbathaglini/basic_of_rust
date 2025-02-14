// DESCRIÇÃO
// 

fn main(){
    
    let condicao = eh_verdadeiro(true);
    println!("eh verdadeiro? {}", condicao);

    let condicao = eh_verdadeiro(false);
    println!("eh verdadeiro? {}", condicao);
     
}


fn eh_verdadeiro(condicao:bool) -> &'static str{
    let booleano_str = if condicao {
        "VERDADEIRO"
    } else {
        "FALSO"
    };

    booleano_str
}
