fn main(){
    let a = 10; //imutável
    println!("let a = {a} -> variável imutável"); //imprime 10

    let mut b = 20;
    println!("let b = {b} -> variável mutável"); //imprime 20

    {
        let a = 2;
        println!("let a = {a}"); //imprime 2

        let b = b + 10;
        println!("let b = {b}"); //imprime 30 (20 da variável fora + 10)
    }

    println!("a = {a} e b = {b}"); //imprime 10 e 20, respectivamente
}