use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to Guess the number ! \n ENTER YOUR GUESS");
    let mut count = 10;
    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        if count == 0 {
            println!("Try again \n Secret Number:{secret_num}");
            break;
        }

        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Enter a valid number");
        count -= 1;
        // conversion from string to unsigned
        let x: u32 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number");
                count += 1;
                continue;
            }
        };

        match x.cmp(&secret_num) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Less => println!("bit low"),
            Ordering::Greater => println!("bit high"),
        }
    }
}
