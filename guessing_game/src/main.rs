use std::io; //std lib::input-output
use std::cmp::Ordering; // compare
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // u32 = unsigned, 32 bit integer, good default choice for small positive number

    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); //mut means mutable

        io::stdin() //std input function in io lib ~ std::io::Stdin
            .read_line(&mut guess)// & REFERENCE call (chapter 4)
            //read line function returns a result value as an enumeration of `Ok` or `Err` (chapter 6)
            .expect("Failed to read line"); // `Err` handling (chapter 9)

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // shadowing guess variable, trim eliminates whitespace at beginning and end
        // parse converts string to other type - can easily cause errors

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}