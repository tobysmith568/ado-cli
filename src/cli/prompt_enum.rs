use std::fmt::Display;

use dialoguer::{console::Term, theme::ColorfulTheme, Select};

pub fn prompt_enum<T: Display>(question: &str, items: &[T]) -> Option<usize> {
    println!("{}", question);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(items)
        .default(0)
        .interact_on_opt(&Term::stdout())
        .unwrap();

    match selection {
        Some(value) => println!("> {}", items[value]),
        None => println!("> No choice"),
    };

    selection
}
