use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

mod day;

fn main() -> std::io::Result<()> {
    // array of 24 day strings, ie: 'day 1', 'day 2', 'day 3', ...
    let items = [
        "day 1", "day 2", "day 3", "day 4", "day 5", "day 6", "day 7", "day 8", "day 9", "day 10",
        "day 11", "day 12", "day 13", "day 14", "day 15", "day 16", "day 17", "day 18", "day 19",
        "day 20", "day 21", "day 22", "day 23", "day 24",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(2) // default to day 3
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => println!("User selected item : {:?}", items[index]),
        None => println!("User did not select anything"),
    }

    match selection {
        Some(0) => day::day_1::main(),
        Some(1) => day::day_2::main(),
        Some(2) => day::day_3::main(),
        _ => println!("User did not select anything"),
    }

    Ok(())
}
