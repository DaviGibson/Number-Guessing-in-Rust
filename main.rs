use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(0..=100);
    println!("Número gerado");

    let mut vitoria = false;
    //let mut palpite: i32;
    let mut input_trim;
    let mut tentativas = 0;
    let mut entra_valida: bool;

    loop {
        println!("Digite seu palpite:");
        let mut input = String::new(); // Cria uma string vazia para armazenar a entrada
        io::stdin().read_line(&mut input).expect("Falha ao ler linha"); // Lê a linha de entrada e armazena em 'input'
        input_trim = input.trim();

        println!("O seu número: {}", input_trim);
        let mut palpite: i32 = 0;
        match input_trim.parse::<i32>() {
            Ok(numero) => {
                // A conversão foi bem-sucedida
                entra_valida = true;
                palpite = numero;
                println!("Número digitado: {}", palpite);
            }
            Err(_) => {
                // A conversão falhou, o input não era um número válido
                entra_valida = false;
                println!("Entrada inválida! Por favor, digite um número inteiro válido.");
            }
        }
        if entra_valida == true {
            if palpite < random_number{
                if palpite < 0 {
                    println!("Você saiu do jogo");
                    break;
                }
                println!("O número digitado é menor que o número gerado");
            } else if palpite > random_number{
                println!("O número digitado é maior que o número gerado");
            } else {
                println!("Você acertou!");
                vitoria = true;
                break;
            }  
            tentativas += 1;
        }
    }

    if vitoria == false{
        println!("Você perdeu");
    } else {
        println!("Você ganhou em {} tentativas", tentativas);
    }

}
