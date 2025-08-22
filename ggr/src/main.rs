use rand::Rng;
use std::io;

fn main() {
    let mut rng =rand::rng();
    let _random_number: i32 = rng.random_range(0..101);
    let _random_number_as_a_string = _random_number.to_string();
    let mut running: bool = true;
    let mut life:u8 = 5;

    println!("{}", _random_number_as_a_string);
    while running {
        println!("Enter a number from 1 to 100");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("faild to read line");

        if input.trim() == _random_number_as_a_string {
            println!("You win!");
            running = false
        } else if *input.trim() >= *_random_number_as_a_string {
            life -= 1;
            println!("The number is to big! You have {} lives left!", life);
        } else if *input.trim() <= *_random_number_as_a_string {
            life -= 1;
            println!("The number is to small! You have {} lives left!", life);
        };

        if life <= 0 {
            println!("You lost!");
            running = false
        }
    };
}