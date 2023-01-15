use dialoguer::{console::Term, theme::ColorfulTheme, Select};

#[derive(Debug)]
pub enum YesNoResult {
    Yes,
    No,
}

pub fn prompt_yes_no(question: &str) -> YesNoResult {
    println!("{}", question);

    let items = vec!["Yes", "No"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap();

    let result = match selection {
        Some(index) => {
            if index == 0 {
                YesNoResult::Yes
            } else {
                YesNoResult::No
            }
        }
        None => YesNoResult::No,
    };

    println!("> {:?}", result);
    result
}
