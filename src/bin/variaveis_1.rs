fn main(){
    let a = 10; //imutável
    println!("let a = {a} -> variável imutável");

    let mut b = 20;
    println!("let b = {b} -> variável mutável");

    b = 150;
    println!("b = {b}");

    //As linhas abaixo gerarão erro porque na linha 5 foi definido o tipo inteiro para 'b', 
    //logo não é possível alterar a mesma variável para string, sem declara-la novamente
    //erro: expected integer, found `&str`
    //b = "abc";
    //println!("b = {b}");
}