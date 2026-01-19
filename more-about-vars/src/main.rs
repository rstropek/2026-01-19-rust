use std::vec;

use rand::Rng;

fn main() {
    let mut rng = rand::rng();

    let value_range = 0..=10;
    let random_number = rng.random_range(value_range);

    // If random_number is > 5, print "win", if < 5 print "lose", else print "draw"
    if random_number > 5 {
        println!("win");
    } else if random_number < 5 {
        println!("lose");
    } else {
        println!("draw");   
    }

    let msg = if random_number > 5 {
        "win"
    } else if random_number < 5 {
        "lose"
    } else {
        "draw"
    };
    println!("{}", msg);

    // match must be exhaustive
    let msg = match random_number {
        //6..=10 => "win",
        //6 | 7 | 8 | 9 | 10 => "win",
        n if n > 5 => "win",
        5 => "draw",
        _ => "lose",
    };
    println!("{}", msg);

    let mut numbers: Vec<u32> = Vec::new();

    // Add 10 random numbers between 1 and 100 to numbers
    for _ in 0..10 {
        let n = rng.random_range(1..=100);
        numbers.push(n);    
    }
    
}
