extern crate num_cpus;
use rand::distributions::{Distribution, Uniform};
use std::cmp::Ordering;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let cpus = num_cpus::get();
    let mut rng = rand::thread_rng();
    let dice = Uniform::from(1..cpus);
    let machine_predict = dice.sample(&mut rng);

    println!("Guess the thread I'm thinking!");
    println!("Machine is thinking...");
    thread::sleep(Duration::from_millis(1000));

    loop {
        println!("Choose a number between 1 and {}", cpus);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&machine_predict) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
