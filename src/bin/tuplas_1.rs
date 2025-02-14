fn main(){

    let tupla = (502, 3.8, "olá");
    println!("O valor da tupla: ({:?}, {:?}, {:?})", tupla.0, tupla.1, tupla.2);

    let (x, y, z) = tupla;
    println!("O valor let (x, y, z) : ({x}, {y}, {z})");


    let x: (i32, i32, f64)  = (100, 200, 3.6);
    
    let cem = x.0;
    let duzentos = x.1;
    let tres_ponto_seis = x.2;
    println!("O valor let (cem, duzentos, tres_ponto_seis) : ({cem}, {duzentos}, {tres_ponto_seis})");

    println!("Tupla vazia: {:?}", ());
}