use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivinhe o número");
    println!("");
    let mut chute: String = String::new();
    let mut rng = rand::thread_rng();
    let numero = rng.gen_range(1..11);
    'repeat: loop {
        chute.clear();
        println!("\nDigite um número: ");

        io::stdin().read_line(&mut chute).expect("\nErro ao ler o número");
        let chute: u32 = match chute.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
            
           match chute.cmp(&numero){
                Ordering::Greater => println!("\nSeu número é maior"),
                Ordering::Less => println!("\nSeu número é menor"),
                Ordering::Equal => {
                    println!("\nVoce acertou!");
                    break 'repeat;
                }
            }
    }
    println!("Digite qualquer tecla para sair...");
    println!("\nAté mais!\n");
    let _ = io::stdin().read_line(&mut chute);
}

