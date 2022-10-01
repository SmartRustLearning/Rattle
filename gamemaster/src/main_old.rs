//mod playground;
//mod sandbox;
mod logic;

use logic::{Exercise, Match};

fn main() {
    // get exercise and preconfig code
    let exs = logic::get_exercises();
    println!("{:?}", exs);
    
    let ex1 = Exercise::from_path(&exs[0].0);
    let ex2 = Exercise::from_path(&exs[2].0);
    let mut player_match = Match::new("P1".to_string(), ex1);
    player_match.join("P2".to_string());
    println!("{:?}", player_match.next_round().get_code_with(r#"println!("hello world")"#.to_string()));

    // compile code
    //playground::main();

    // return result
}