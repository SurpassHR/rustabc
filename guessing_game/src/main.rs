use std::io;
use rand::Rng;
use std::cmp;

fn main() {
    // generated a random number
    let tar = rand::thread_rng().gen_range(1..101);
    // continously guessing number
    loop {
        // get a number string from standard input
        let mut guess : String = String::new();
        println!("Please input a guessing number: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Please input a guessing number: ");
        // print the guessing number
        // println!("The guessing number is: {}.", guess.trim());
        // print the target number
        // println!("The target number is: {}.", tar);
        // compare the guessing number with the given number
        // need to transform the string type number into numeric
        // if input number is not a valid number then continue
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => { num },
            Err(_) => { continue },
        };
        match guess.cmp(&tar) {
            cmp::Ordering::Less => { println!("Too small.") },
            cmp::Ordering::Greater => { println!("Too big.") },
            cmp::Ordering::Equal => {
                println!("You win!");
                break;
            },
        };
    }
}