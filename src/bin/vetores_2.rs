// DESCRIÇÃO
// Declarando vetor com tamanho fixo [&'static str;12]
// Declarando vetor com tamanho dinâmico Vec<&'static str>
// Declarando vetor com tamanho dinâmico Vec<String>

fn main(){
    
    let meses = get_meses(); // [&'static str;12]
    println!("Meses: {:?}", meses);

    let nomes = get_nomes(); // Vec<&'static str>
    println!("Nomes: {:?}", nomes);

    let nomes_v2 = get_nomes_v2(); // Vec<String>
    println!("Nomes v2: {:?}", nomes_v2);
    
}

fn get_meses() -> [&'static str;12] {
    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho",
    "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];
    meses
}

fn get_nomes() -> Vec<&'static str> {
    let nomes = vec!["Camila", "Vitor", "Alice", "João"];
    nomes
}

fn get_nomes_v2() -> Vec<String> {
    let mut nomes: Vec<String> = Vec::new();

    nomes.push(String::from("Camila"));
    nomes.push(String::from("Vitor"));
    nomes.push("Alice".to_string());
    nomes.push("João".to_string());

    nomes
}

