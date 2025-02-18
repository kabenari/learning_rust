use rand::Rng;
use std::cmp::Ordering;
use std::io; //  io library we are introducing //ranf lib

fn main() {
    println!("Guess the number");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new(); // mutable variabl

        io::stdin()
            .read_line(&mut guess) //getline in c++
            .expect("Failed to read line");

        println!("guessed word is :{}", guess);
        println!("the generated number is {}", secret_number);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("the secret is :{secret_number}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
