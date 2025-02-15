fn check_guess(guess: i32, secret: i32) -> i32{
    match guess{
        _ if guess < secret => -1,
        _ if guess > secret => 1,
        _ if guess == secret => 0,
        _ => 999,
    }
}

fn main() {
    let mut secret_num = 77;
    let mut counter = 0;
    loop{
        counter += 1;
        let mut guess_num =  77;

        let response = check_guess(guess_num, secret_num);

        if response == -1{
            println!("Guess Too Low!");
        }
        else if response == 1{
            println!("Guess Too High!");
        }
        
        if response == 0{
            println!("Correct!");
            break;
        }
    }
    println!("Number of tries: {}", counter);
}
