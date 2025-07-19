// MateiDevel on Github

use std::io;

// Source : The Rust Programing Language Book 
// Chapter 5.1 - Defining and Instantiating Structs
// A struct, or structure, is a custom data type that lets you name and package together multiple
// related values that make up a meaningful group.

struct Player
{
    name : String,
    score : i32,
    outfit : bool
}

impl Player
{
    fn select_name(name : String) -> Player
    {
        Player
        {
            name: name.trim().to_string(),
            score: 0
        }
    }

    fn increase_score(&mut self, score : i32)
    {
        self.score += score;
    }
}

fn main()
{
    let mut player = Player::select_name("Matei".to_string());
    player.outfit = true;
    player.increase_score(30);
}