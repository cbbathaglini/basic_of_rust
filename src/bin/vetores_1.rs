// DESCRIÇÃO
// Dado o número do mês de aniversário, deve retornar o nome do mês correspondente.

fn main(){
    
    let mesAniver = get_mes_aniversario(6);
    println!("Mês do aniver: {mesAniver}");
}

fn get_mes_aniversario(mes:usize) -> &'static str {
    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho",
    "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];

    //println!("pos[0]: {:?}", meses[0]); //imprime janeiro
    //println!("pos[11]: {:?}", meses[11]); // imprime dezembro

    if mes >= 1 && mes <= 12 {
        meses[mes - 1]
    } else {
        "Mês inválido"
    }
}
