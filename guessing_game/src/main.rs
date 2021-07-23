use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("input:");

        let mut guess = String::new();
    
        io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Hello, {}!", guess);
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Less!"),
            std::cmp::Ordering::Equal => {
                println!("Equal!");
                break;
            },
            std::cmp::Ordering::Greater => println!("Greater!"),
        }
    }    
}
