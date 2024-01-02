use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut check: bool = false;
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    let mut counter: u32 = 0;
    let mut helper: u32 = rand::thread_rng().gen_range(secret_number - 10..secret_number + 10);


    println!("Indovina il numero!");

    while !check {
        println!("Inserisci la tua ipotesi");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Lettura riga fallita");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Hai ipotizzato il {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                counter += 1;
                println!("Troppo piccolo");
            }
            Ordering::Greater => {
                counter += 1;
                println!("Troppo grande");
            }
            Ordering::Equal => {
                println!("Hai vinto! per indovinare il numero {} hai impiegato {} tentativi", secret_number, counter);
                check = true;
            }
        }

        
        if counter == 3 && !check{
            print!("\n \n Il numero è intorno al {} \n \n", helper)
        }

        if counter == 6 && !check{
            helper = rand::thread_rng().gen_range(secret_number - 4..secret_number + 4);
            print!("\n \n Il numero è intorno al {} \n \n", helper)
        }
        
        if counter == 10 && !check{
            println!("Mi dispiace hai perso, il numero era: {}", secret_number);
            break;
        }
    }
}
