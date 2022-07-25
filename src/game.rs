use crate::words_list;
use std::io;

pub fn read_user_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.to_lowercase().trim().to_string())
}

pub fn run() {
    let mamium_tries = 6 as i32;
    let mut attempts = 0;
    println!("lets start the wordle game");
    println!("Maximum tries: {0}; left tries - {0}", mamium_tries);
    let word = words_list::WordList::new();
    let answer = word.pick_a_word();
    // println!("{}", answer);
    loop {
        println!("Please input your suggestion word");
        let guess = read_user_input().unwrap();
        // println!("{}",guess);
        // println!("{}",attempts);
        let hints = match_guess_letters(&guess, &answer);
        pretty_print(&guess);
        pretty_print(&hints);
        attempts += 1;
        // println!("{}",attempts);
        if guess == answer {
            println!("You Win!");
            return;
        }

        if attempts < 6 {
            println!("{} more trie(s)", mamium_tries - attempts);
        } else {
            break;
        }
    }
    println!("Game over, the answer was {}", answer);
}

fn match_guess_letters(guess: &str, answer: &str) -> String{
    let green = '🟩';
    let grey = '⬛';
    let yellow = '🟨';
    let answer_bytes = answer.as_bytes();
    let mut hint_answer = String::new();
    for (i, a) in guess.bytes().enumerate() {
        let x = if a == answer_bytes[i] {
            green
        } else if answer_bytes.contains(&a) {
            yellow
        } else {
            grey
        };
        hint_answer.push(x);
    }
    hint_answer

}
pub fn pretty_print(s: &str) {
    print!("{}", s.chars().next().unwrap());
    s.chars().skip(1).for_each(|c| print!("\t{}", c));
    println!("");
}