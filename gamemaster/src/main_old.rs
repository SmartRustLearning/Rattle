mod playground;
mod sandbox;
mod logic;

use logic::{Exercise, Match};

fn main() {
    // get exercise and preconfig code
    let ex = Exercise::from_path("./exercises/ex1");
    let mut player_match = Match::new("P1".to_string(), ex);
    player_match.join("P2".to_string());
    println!("{:?}", player_match.next_round().get_code_with(r#"println!("hello world")"#.to_string()));

    // compile code
    //playground::main();

    // return result
}