use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Lance um numero!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    print!("{}", secret_number);
    loop {
        println!("================================================================");
        println!("Por favor digite seu lance.");

        let mut guess = String::new(); //variavel mutável

        io::stdin()
            .read_line(&mut guess)
            .expect("Falhou, não consegui ler seu lance");

        let guess: u32 = guess.trim().parse().expect("Por favor digite um numero");
        println!("Seu lance: {guess}");
        println!("================================================================");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Tente um numero maior"),
            Ordering::Greater => println!("Tente um numero Menor"),
            Ordering::Equal => {
                println!("Você venceu! 🚀");
                break;
            }
        }
    }
}
