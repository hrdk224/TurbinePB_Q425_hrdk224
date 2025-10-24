use std::io;
fn main() {
    let mut num = String::new();
    println!("Welcome to counter!\n Enter a number: ");
    io::stdin().read_line(&mut num).expect("failed to readline");
    let mut num: i32 = num.trim().parse().expect("Please type a number");

    println!("What operation do you want to perform: \n + --> Increment \n - --> Decrement");

    // if op == "+" {
    //     num = num + 1;
    //     println!("incremented output:{}", num);
    // }

    // if op == "-" {
    //     num = num - 1;
    //     println!("decremented output:{}", num);
    // }
    loop {
        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("failed to readline");
        let op: &str = op.trim();
        match op {
            "+" => {
                num = num + 1;
                println!("incremented output:{}", num);
                break;
            }
            "-" => {
                num = num - 1;
                println!("decremented output:{}", num);
                break;
            }
            _ => {
                println!("input either + or -");
            }
        }
    }
}
