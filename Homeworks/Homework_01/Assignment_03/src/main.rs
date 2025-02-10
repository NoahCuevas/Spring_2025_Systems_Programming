use std::io;
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        return 0
    } else if guess > secret {
        return 1
    } else {
        return -1
    }
}

fn main() {
    let secret_num = 62;
    let mut attempts = 0;

    println!("Guess a number 1 - 100");
    loop {
        println!("Enter guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        println!();
        attempts += 1;
        let result = check_guess(guess, secret_num);

        if result == 0 {
            println!("Correct! You guessed the number in {} attempts.", attempts);
            break;
        } else if result == 1 {
            println!("Too high guess again.");
        } else {
            println!("Too low guess again.");
        }
    }
}
