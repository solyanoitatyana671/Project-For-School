// This Rust code snippet demonstrates how to implement a simple game of rock-paper-scissors.
fn main() {
    let user_choice = prompt_user();
    let computer_choice = choose_random();

    if user_choice == computer_choice {
        println!("It's a tie! Play again.");
    } else if (user_choice.to_lowercase().compare(&computer_choice)) {
        println!("You win!");
    } else {
        println!("Computer wins!");
    }

    println!("Game over. Thanks for playing!");
}

fn prompt_user() -> String {
    let mut user_answer = String::new();
    println!("Enter your choice: rock (R), paper (P), or scissors (S): ");
    std::io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read line");
    user_answer.trim().to_lowercase()
}

fn choose_random() -> String {
    let mut computer_choice = String::new();
    println!("Computer's turn: rock (R), paper (P), or scissors (S)");
    std::io::stdin()
        .read_line(&mut computer_choice)
        .expect("Failed to read line");
    computer_choice.trim().to_lowercase()
}
