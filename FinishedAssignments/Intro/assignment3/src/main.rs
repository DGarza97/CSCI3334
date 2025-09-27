fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret{
        0
    } else if guess > secret{
        1
    } else{
        -1
    }
}

fn main() {
    let mut secret = 10;
    let mut guess = 20;
    let mut counter = 0;

    loop{
        let mut guess_evaluation = check_guess(guess, secret);

        counter += 1;

        if guess_evaluation == 0{
            println!("Your guess was correct!");
            break;
        } else if guess_evaluation == 1{
            println!("Your guess was too high!");
            guess -= 1;
        } else if guess_evaluation == -1{
            println!("Your guess was too low!");
            guess += 1;
        }
    }
    println!("It took {counter} guesses to find the answer!");
}