use std::io;

fn main() {
    println!("Hello world!");
    antonystarr();
}

fn antonystarr() {
    let mut opt = String::new();
    let mut x = String::new();
    let base_value: u32 = 10;  // Valor base fixo

    println!("O valor base é {}! Altere-o:", base_value);
    
    io::stdin()
        .read_line(&mut x)
        .expect("Erro! Caractere inválido ou campo vazio!");
    
    // converte para u32, se dar ruim, usa o valor base
    let x: u32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => base_value,
    };

    println!("Você alterou para {}", x);

    println!("Deseja multiplicar esse número por outro? S/N");
    
    io::stdin()
        .read_line(&mut opt)
        .expect("Inválido, tente novamente!");

    // transforma em maiúsculas para comparação
    let opt = opt.trim().to_uppercase();
    
    if opt == "N" {
        println!("Fechando programa...");
    } else if opt == "S" {
        cont(x);
    } else {
        println!("Opção inválida. Encerrando programa.");
    }
}

fn cont(base_value: u32) {
    
    println!("Por favor, repita o número desejado: ");
    
    //variável mutável para entrada do usuário
    let mut input = String::new();
    
    // Lê a linha de entrada do usuário
    io::stdin().read_line(&mut input).expect("Ocorreu um erro ao ler a entrada.");
    
    // Converte a entrada para um número inteiro
    let x2: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada inválida. Usando 0 como valor padrão.");
            0
        },
    };

    // Inicializa variáveis para a multiplicação
    let mut y = 1;
    let mut resultado;
    
    // Loop para multiplicar 
    while y <= 100 {
        resultado = y * x2;
        println!("Multiplicando {} por {} é igual a {}", x2, y, resultado);
        y += 1;
    }
}
