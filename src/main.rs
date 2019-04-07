use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //declare vars needed
    //random gen range is 1 to 101 because rand::thread_rng is inclusive on the lower bound but exclusive on the upper bound
    let mut lives = 10;
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //beginning
    println!("Guess a number from 1 to 100! You have {} guesses!", lives);
    println!("Please guess now:");

    //start game loop
    loop{
    
    //create the guess var
    let mut guess = String::new();   

    //read in the guess
    io::stdin().read_line(&mut guess).expect("Failed to read line...");

    //convert the guess to an u32 int for comparison
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    //print your guess
    println!("You guessed the number {}!", guess);
    
        //check your guess against the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
             lives = lives-1;
            }
            Ordering::Greater => {
                println!("Too big!");
             lives = lives-1;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        //Displays how many lives you have left!
        println!("You have {} lives left!", lives);

        //check how many lives you have left
        match lives.cmp(&0){
            Ordering::Less => {

            }
            Ordering::Greater => {

            }
            Ordering::Equal => {
                println!("You Lose...");
                break;
            }
        }
    }
}


