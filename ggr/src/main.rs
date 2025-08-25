use rand::Rng;
use std::io;

fn main() {
    let mut rng =rand::thread_rng();
    let _random_number: i32 = rng.gen_range(0..101);
    let mut running: bool = true;
    let mut life:u8 = 5;

    while running {
        println!("Enter a number from 1 to 100");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("faild to read line");
        let input_as_an_integer:i32 = input.trim().parse().expect("Faild to phrase string");

        if input_as_an_integer == _random_number {
            println!("You win!");
            running = false
        } else if input_as_an_integer >= _random_number {
            life -= 1;
            println!("The number is to big! You have {} lives left!", life);
        } else {
            life -= 1;
            println!("The number is to small! You have {} lives left!", life);
        };

        if life <= 0 {
            println!("You lost!");
            running = false
        };
    };
}