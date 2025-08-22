use rand::Rng;

fn main() {
    let mut rng =rand::thread_rng();

    let random_number: u32 = rng.gen_range(0..100);
    println!("Random number between 0 and 99: {}", random_number);
}
