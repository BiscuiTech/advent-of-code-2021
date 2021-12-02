use std::fs;

use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() -> std::io::Result<()> {
    // array of 24 day strings, ie: 'day 1', 'day 2', 'day 3', ...
    let items = [
        "day 1", "day 2", "day 3", "day 4", "day 5", "day 6", "day 7", "day 8", "day 9", "day 10",
        "day 11", "day 12", "day 13", "day 14", "day 15", "day 16", "day 17", "day 18", "day 19",
        "day 20", "day 21", "day 22", "day 23", "day 24",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => println!("User selected item : {:?}", items[index]),
        None => println!("User did not select anything"),
    }

    match selection {
        Some(0) => day_1(),
        _ => println!("User did not select anything"),
    }

    Ok(())
}

fn day_1() {
    // read file from disk
    let contents = fs::read_to_string("src/day1/readings.txt").unwrap();
    println!("{}", contents);
}
