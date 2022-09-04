use crate::GameState;

struct Location {
    description: &'static str,
    tag: &'static str,
}

const LOCATIONS: &[Location] = &[
    Location {
        description: "an open field",
        tag: "field"
    },
    Location {
        description: "a little cave",
        tag: "cave"
    }
];

pub(crate) fn execute_look(noun: Option<&str>, state: &GameState) {
    match noun {
        Some("around") => println!("You are in {}.", LOCATIONS[state.player_location].description),
        Some(_) | None => println!("I don't understand what you want to see."),
    }
}

pub(crate) fn execute_go(noun: Option<&str>, state: &mut GameState) {
    if let Some(noun) = noun {
        let mut iter = LOCATIONS.iter().enumerate().filter(|(_, l)| l.tag == noun);

        if let Some((idx, location)) = iter.next() {
            if idx == state.player_location {
                println!("You can't get much closer than this.");
            } else {
                println!("OK.");
                state.player_location = idx;
                execute_look(Some("around"), state);
            }
            return;
        }
    }

    println!("I don't understand where you want to go.");
}