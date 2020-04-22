extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

struct Hunch {
    value: i32,
}

impl Hunch {
    fn new(value: i32) -> Result<Hunch, String> {
        if value > 100 || value < 1 {
            return Err("Digite um número apenas entre 1 e 100".to_string());
        };
        Ok(Hunch { value })
    }

    fn get_value(&self) -> u32 {
        self.value as u32
    }
}

fn main() {
    println!("Adivinhe o número!!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Digite seu palpite.");

        let mut value = String::new();

        io::stdin().read_line(&mut value).expect("Error");

        let value: i32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Digite um número!");
                continue;
            }
        };

        let hunch = match Hunch::new(value) {
            Ok(value) => value,
            Err(e) => panic!(e),
        };

        println!("Você disse: {}", hunch.get_value());

        match hunch.get_value().cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!!!"),
            Ordering::Greater => println!("Muito alto!!!"),
            Ordering::Equal => {
                println!("Você acertou!!!");
                break;
            }
        }
    }
}
