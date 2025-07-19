// MateiDevel on Github

mod structs;

use std::io;

// Source : The Rust Programing Language Book 
// Chapter 6.1 - Defining an Enum 
// Enums, or enumerations, allow you to define a type by enumerating its possible variants.

enum Voicelines
{
    Line1,
    Line2,
    Line3,
    Line4,
}

fn say_lines(lines: Voicelines)
{
    match lines
    {
       Voicelines::Line1 => println!("hello"),
       Voicelines::Line2 => println!("world,"),
       Voicelines::Line3 => println!("im"),
       Voicelines::Line4 => println!("alive!"),
    }
}

fn main() {
    println!("select which line would you like me to say, or i can say all.");
    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();

    if line.trim() == "Line1"
    {
        say_lines(Voicelines::Line1);
    }
    if line.trim() == "Line2"
    {
        say_lines(Voicelines::Line1);
    }
    if line.trim() == "Line3"
    {
        say_lines(Voicelines::Line1);
    }
    if line.trim() == "Line4"
    {
        say_lines(Voicelines::Line1);
    }
    if line.trim() == "all"
    {
        say_lines(Voicelines::Line1);
        say_lines(Voicelines::Line2);
        say_lines(Voicelines::Line3);
        say_lines(Voicelines::Line4);
    }
}
