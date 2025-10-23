//use std::io;
use std::thread;
use std::time::Duration;
fn main() {
    //** if you want to type any other word uncomment the code and replace "hello world" with word **
    // let mut word = String::new();
    // io::stdin()
    //     .read_line(&mut word)
    //     .expect("failed to readline");
    let mut out = String::new();

    for c in "helloworld".chars() {
        // println!("{c}");
        for a in 'a'..=c {
            println!("{out}{a}");
            thread::sleep(Duration::from_millis(5)); //change the speed of output here
            if a == c {
                out.push(c);
            }
        }
    }
}
