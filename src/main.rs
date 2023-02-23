use std::io;
use rand::Rng;

fn main() {
    guessing_game();
}

fn guessing_game() {
    let mut input = String::new();
    let mut rng = rand::thread_rng();
    let mut guess = rng.gen_range(1..101);
    let mut failed: Vec<i32> = Vec::new();
    let mut l = 1; let mut u = 101; let mut guess_count = 0;

    while input.trim().to_uppercase() != "Y" {
        for i in &failed {
            if guess == * i { guess = rng.gen_range(l..u); }
        }
        println!("My guess: {}", guess);
        input.clear();

        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_uppercase().as_str() {
            "H" => {
                guess_count += 1;
                l = guess + 1;
                if u < l { u -= guess_count; }
                failed.push(guess);
                guess = rng.gen_range(l..u);
            },
            "L" => {
                guess_count += 1;
                if u == l { u = guess_count; }
                u = guess;
                failed.push(guess);
                guess = rng.gen_range(l..u);
            },
            "Y" => {
                println!("I did it in {} guesses!\nPress X to go again...", guess_count);
                io::stdin().read_line(&mut input).unwrap();

                if input.trim() == "X" { guessing_game(); }
            },
            _ => { println!("Invalid input. Please enter H, L, or Y."); },
        }
    }
}