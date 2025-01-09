use std::io::{self,Write}; //This is use for handling input and output..
use rand::Rng; // Use to generate a random number..
fn main(){
    println!("Welcome to Excel Guessing Game.");
//This is use to generate the secret_number..
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Can You Please Guess The Numerical Value Between 1 and 100, That I Have Generated.");

    let mut attempts = 0;

// This section continues to ask the user until the user get the secret number..
    loop{
        println!("Please Input Your Guess!");
        io::stdout().flush().unwrap();
        let mut guessed_value = String::new();
        io::stdin()
        .read_line(&mut guessed_value)
        .expect("Failed To Read Line!");  
    let mut guessed_value:u32 = match guessed_value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please Try Inputing A Postive Value Between 1 and 100.");
            continue; // Here this is use to check if what your inputing is valid..
        }
    };

        if guessed_value < 1 || guessed_value > 100 {                  // This is use to ensure that the number you are guessing is not above or below the range 1 and 100..
            println!("Plese Guess A Number Between 1 and 100.");
            continue; // I use it to ensure that if guess is out of range one can try again i.e it will ask again..
        }

    attempts += 1;

    println!("Your Guessed Value Is: {}.",guessed_value);

// This section is comparing the guessed_value with the secret number
    if guessed_value < secret_number{
        println!("Your Guessed Value Is Less Than The Value Generated! Please Try Using A Higher Number.");
    } else if guessed_value > secret_number{
        println!("The Value You Guessed Is Greater Than The Value Generated! Please Try Using A Lower Number.");
    } else {
        println!("Congratulations you guessed the right value: {}.",secret_number);
        println!("Welldone You Took {} Attempts.",attempts);
    break; // This means the exist of the loop once the guesser as gotten the secret or generated number..
}
}
}




