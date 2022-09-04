use std::io;
use std::io::Write;

use location::*;

mod location;
mod object;

struct GameState {
    player_location: usize,
}

fn parse_and_execute(input: &str, state: &mut GameState) -> bool {
    let mut split = input.split(" ");
    let verb = split.next();
    let noun = split.next();

    match verb {
        Some("quit") => return false,
        Some("look") => execute_look(noun, state),
        Some("go") => execute_go(noun, state),
        Some("") => (),
        Some(unknown) => println!("I don't know how to '{}'", unknown),
        None => (),
    }

    true
}

fn get_input(buffer: &mut String) -> Result<(), io::Error> {
    print!("\n--> ");
    io::stdout().flush()?;

    buffer.clear();
    io::stdin().read_line(buffer)?;
    buffer.truncate(buffer.trim_end().len());

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let mut input = "look around".to_string();
    let mut state = GameState {
        player_location: 0
    };

    println!("Welcome to Little Cave Adventure.");

    loop {
        if !parse_and_execute(&input, &mut state) { break; }
        get_input(&mut input)?;
    }

    println!("Bye!");
    Ok(())
}
